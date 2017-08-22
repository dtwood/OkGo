use core::mem;

use ignition;
use firmware_common::adc::adc_to_millivolts;
use bare_metal::CriticalSection;
use md_5::Md5;
use hmac::{Hmac, Mac};
use firmware_common::{key, rfm};
use io;
use stm32f0xx;
use f0::gpio::{Gpio, Port};
use f0::spi::Spi;
use nb;

const REQ_PACKET_LEN: usize = 11;
const RSP_PACKET_LEN: usize = 17;

/// Radio tx power in dBm
const RADIO_POWER_DBM: u8 = 10;

/// Convert raw ADC value to continuity ohms
fn adc_to_ohms(raw: u16) -> u8 {
    let mut millivolts = adc_to_millivolts(raw);

    if millivolts >= 3300 {
        millivolts = 3299; /* Avoid negatives or div by zero */
    }

    /* Remove 48mV offset caused by diode reverse current */
    if millivolts >= 48 {
        millivolts -= 48;
    } else {
        millivolts = 0;
    }

    let ohms = millivolts * 1000 / (3300 - millivolts);

    if ohms >= 10000 {
        return 255; /* magic value meaning open */
    } else if ohms >= 255 {
        return 254; /* magic value meaning high but not open */
    } else {
        return ohms as u8;
    }
}

output!(RFM_NSS, A, 15);

static RFM_SPI: Spi = Spi {
    sck: Gpio {
        port: Port::B,
        pin: 3,
    },
    miso: Gpio {
        port: Port::B,
        pin: 4,
    },
    mosi: Gpio {
        port: Port::B,
        pin: 5,
    },
};

pub fn init(cs: &CriticalSection) {
    // Clock SPI1 peripheral and setup GPIOs appropriately:
    // NSS, SCK, MOSI, RESET are outputs,
    // MISO is input.
    // SPI setup is done in rfm95w.c
    stm32f0xx::RCC
        .borrow(cs)
        .apb2enr
        .write(|w| w.spi1en().set_bit());

    // Make sure NSS doesn't blip when we enable it:
    RFM_NSS.clear(cs);
    RFM_NSS.setup(cs);
    RFM_SPI.setup(cs);

    // Run RFM95W initialization
    unsafe {
        rfm::rfm_initialise(
            stm32f0xx::SPI1.get() as u32,
            RFM_NSS.gpio.port as u32,
            RFM_NSS.gpio.pin,
        );
        rfm::rfm_setfreq(rfm::Frf::Frf868 as u32);
        rfm::rfm_setpower(RADIO_POWER_DBM);
    }
}

/// Transmit a packet to control based on the contents of state
pub fn transmit(cs: &CriticalSection, state: &ignition::State) {
    let mut buf = [0; RSP_PACKET_LEN];

    buf[0] = state.packet_rssi;

    let mut adc_val: u32 = adc_to_millivolts(io::BATT_MON.read(cs));

    adc_val = adc_val * 133 / 33;
    if (adc_val / 10) % 10 >= 5
    /* Round instead of truncate */
    {
        buf[1] = (adc_val / 100 + 1) as u8;
    } else {
        buf[1] = (adc_val / 100) as u8;
    }
    let status: u8 = ((state.armed as u8) << 4) | ((state.fire_ch4 as u8) << 3) |
        ((state.fire_ch3 as u8) << 2) | ((state.fire_ch2 as u8) << 1) |
        (state.fire_ch1 as u8);
    buf[2] = status;

    buf[3] = adc_to_ohms(io::CONT_CH1.read(cs));
    buf[4] = adc_to_ohms(io::CONT_CH2.read(cs));
    buf[5] = adc_to_ohms(io::CONT_CH3.read(cs));
    buf[6] = adc_to_ohms(io::CONT_CH4.read(cs));

    /* Generate message HMAC signature */
    let mut mac = Hmac::<Md5>::new(key::get_key());
    mac.input(&buf[0..(RSP_PACKET_LEN - 10)]);
    buf[(RSP_PACKET_LEN - 10)..].clone_from_slice(mac.result().code());

    unsafe {
        rfm::rfm_transmit(buf.as_ptr(), RSP_PACKET_LEN as u8);
    }
}

pub fn make_ready() {
    unsafe {
        rfm::rfm_receive_async(REQ_PACKET_LEN as u8);
    }
}

fn parse_packet(buf: [u8; REQ_PACKET_LEN]) -> Result<ignition::State, ReceiveError> {
    let mut mac = Hmac::<Md5>::new(key::get_key());
    mac.input(&buf[0..(REQ_PACKET_LEN - 10)]);
    if !mac.verify(&buf[(REQ_PACKET_LEN - 10)..]) {
        return Err(ReceiveError::InvalidHash);
    }

    let command = buf[0];

    let armed = command & (1 << 4) != 0;
    let beep_volume = (command >> 5) & 0x07;
    let rssi = unsafe { rfm::rfm_getrssi() };

    Ok(if armed {
        ignition::State {
            beep_volume: beep_volume,
            beep_start: 0,
            packet_rssi: rssi,
            armed: armed,
            fire_ch1: command & (1 << 0) != 0,
            fire_ch2: command & (1 << 1) != 0,
            fire_ch3: command & (1 << 2) != 0,
            fire_ch4: command & (1 << 3) != 0,
        }
    } else {
        ignition::State {
            beep_volume: beep_volume,
            beep_start: 0,
            packet_rssi: rssi,
            armed: armed,
            fire_ch1: false,
            fire_ch2: false,
            fire_ch3: false,
            fire_ch4: false,
        }
    })
}

pub enum ReceiveError {
    ReceiveError,
    InvalidHash,
}

/// Retrieve and parse a packet received in async receive
pub fn receive_async() -> nb::Result<ignition::State, ReceiveError> {
    if unsafe { rfm::rfm_packet_waiting() } {
        let mut rx_buf: [u8; REQ_PACKET_LEN] = unsafe { mem::uninitialized() };

        if unsafe { rfm::rfm_packet_retrieve(rx_buf.as_mut_ptr(), REQ_PACKET_LEN as u8) } {
            parse_packet(rx_buf).map_err(|e| nb::Error::Other(e))
        } else {
            Err(nb::Error::Other(ReceiveError::ReceiveError))
        }
    } else {
        Err(nb::Error::WouldBlock)
    }
}
