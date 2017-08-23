use core::mem;

use md_5::Md5;
use hmac::{Hmac, Mac};
use firmware_common::{key, rfm};
use stm32f0xx;
use f0::gpio::{Gpio, Port};
use f0::spi::Spi;
use nb;

const REQ_PACKET_LEN: usize = 11;
const CFM_PACKET_LEN: usize = 17;

/// Radio tx power in dBm
const RADIO_POWER_DBM: u8 = 10;

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

pub struct ReqPacket {
    pub rssi: u8,
    pub armed: bool,
    pub fire_ch1: bool,
    pub fire_ch2: bool,
    pub fire_ch3: bool,
    pub fire_ch4: bool,
    pub beep_start: u32,
    pub beep_volume: u8,
}

pub struct CfmPacket {
    pub received_rssi: u8,
    pub battery_voltage: u8,
    pub armed: bool,
    pub fire_ch1: bool,
    pub fire_ch2: bool,
    pub fire_ch3: bool,
    pub fire_ch4: bool,
    pub cont_ch1: u8,
    pub cont_ch2: u8,
    pub cont_ch3: u8,
    pub cont_ch4: u8,
}

pub fn init(p: &::init::Peripherals) {
    // Clock SPI1 peripheral and setup GPIOs appropriately:
    // NSS, SCK, MOSI, RESET are outputs,
    // MISO is input.
    // SPI setup is done in rfm95w.c
    p.RCC.apb2enr.write(|w| w.spi1en().set_bit());

    // Make sure NSS doesn't blip when we enable it:
    RFM_NSS.clear();
    RFM_NSS.setup();
    RFM_SPI.setup();

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
pub fn transmit(packet: &CfmPacket) {
    let mut buf = [0; CFM_PACKET_LEN];

    buf[0] = packet.received_rssi;
    buf[1] = packet.battery_voltage;

    buf[2] = ((packet.armed as u8) << 4) | ((packet.fire_ch4 as u8) << 3) |
        ((packet.fire_ch3 as u8) << 2) | ((packet.fire_ch2 as u8) << 1) |
        (packet.fire_ch1 as u8);

    buf[3] = packet.cont_ch1;
    buf[4] = packet.cont_ch2;
    buf[5] = packet.cont_ch3;
    buf[6] = packet.cont_ch4;

    /* Generate message HMAC signature */
    let mut mac = Hmac::<Md5>::new(key::get_key());
    mac.input(&buf[0..(CFM_PACKET_LEN - 10)]);
    buf[(CFM_PACKET_LEN - 10)..].clone_from_slice(mac.result().code());

    unsafe {
        rfm::rfm_transmit(buf.as_ptr(), CFM_PACKET_LEN as u8);
    }
}

pub fn make_ready() {
    unsafe {
        rfm::rfm_receive_async(REQ_PACKET_LEN as u8);
    }
}

fn parse_packet(buf: [u8; REQ_PACKET_LEN]) -> Result<ReqPacket, ReceiveError> {
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
        ReqPacket {
            rssi: rssi,
            beep_volume: beep_volume,
            beep_start: 0,
            armed: true,
            fire_ch1: command & 1 != 0,
            fire_ch2: command & (1 << 1) != 0,
            fire_ch3: command & (1 << 2) != 0,
            fire_ch4: command & (1 << 3) != 0,
        }
    } else {
        ReqPacket {
            rssi: rssi,
            beep_volume: beep_volume,
            beep_start: 0,
            armed: false,
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
pub fn receive_async() -> nb::Result<ReqPacket, ReceiveError> {
    if unsafe { rfm::rfm_packet_waiting() } {
        let mut rx_buf: [u8; REQ_PACKET_LEN] = unsafe { mem::uninitialized() };

        if unsafe { rfm::rfm_packet_retrieve(rx_buf.as_mut_ptr(), REQ_PACKET_LEN as u8) } {
            parse_packet(rx_buf).map_err(nb::Error::Other)
        } else {
            Err(nb::Error::Other(ReceiveError::ReceiveError))
        }
    } else {
        Err(nb::Error::WouldBlock)
    }
}
