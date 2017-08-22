use control;
use core::mem;
use md_5::Md5;
use hmac::{Hmac, Mac};
use firmware_common::{key, rfm};
use nb;
use radio;
use stm32f0xx;
use bare_metal::CriticalSection;
use f0::spi::Spi;
use f0::gpio::{Gpio, Port};

const REQ_PACKET_LEN: usize = 11;
const RSP_PACKET_LEN: usize = 17;

/// Radio tx power in dBm
const RADIO_POWER_DBM: u8 = 10;

/// Control radio state structure
#[repr(C)]
pub struct State {
    /// RSSI *of* the incoming packet, filled by rx
    pub packet_rssi: u8,
    /// RSSI *in* the incoming packet
    pub rx_rssi: u8,
    pub rx_voltage: u8,
    pub rx_status: u8,
    pub rx_cont1: u8,
    pub rx_cont2: u8,
    pub rx_cont3: u8,
    pub rx_cont4: u8,
}

pub enum ReceiveError {
    ReceiveError,
    InvalidHash,
}

output!(RFM_NSS, A, 4);
// output!(RFM_RESET, A, 3);

static RFM_SPI: Spi = Spi {
    sck: Gpio {
        port: Port::A,
        pin: 5,
    },
    miso: Gpio {
        port: Port::A,
        pin: 6,
    },
    mosi: Gpio {
        port: Port::A,
        pin: 7,
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
    RFM_NSS.set(cs);
    RFM_NSS.setup(cs);
    RFM_SPI.setup(cs);

    // Bring the radio out of reset: rfm_init will wait for it to warm up
    // RFM_RESET.setup(cs);
    // RFM_RESET.set(cs);
    // By default leave RFM_RESET high-Z as requested by datasheet

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

pub fn make_ready() {
    unsafe {
        rfm::rfm_receive_async(RSP_PACKET_LEN as u8);
    }
}

/// Transmit a packet to control based on the contents of state
pub fn transmit(state: &control::State) {
    let mut buf = [0; REQ_PACKET_LEN];

    buf[0] |= state.beep_volume << 5;
    buf[0] |= (state.armed as u8) << 4;
    buf[0] |= ((state.ch4_status == control::ChannelStatus::Fire) as u8) << 3;
    buf[0] |= ((state.ch3_status == control::ChannelStatus::Fire) as u8) << 2;
    buf[0] |= ((state.ch2_status == control::ChannelStatus::Fire) as u8) << 1;
    buf[0] |= ((state.ch1_status == control::ChannelStatus::Fire) as u8) << 0;

    // Generate message HMAC signature
    let mut mac = Hmac::<Md5>::new(key::get_key());
    mac.input(&buf[0..(REQ_PACKET_LEN - 10)]);
    buf[(REQ_PACKET_LEN - 10)..].clone_from_slice(mac.result().code());

    unsafe {
        rfm::rfm_transmit(buf.as_ptr(), REQ_PACKET_LEN as u8);
    }
}

fn parse_packet(buf: [u8; RSP_PACKET_LEN]) -> Result<radio::State, ReceiveError> {
    let mut mac = Hmac::<Md5>::new(key::get_key());
    mac.input(&buf[0..(RSP_PACKET_LEN - 10)]);
    if !mac.verify(&buf[(RSP_PACKET_LEN - 10)..]) {
        return Err(ReceiveError::InvalidHash);
    }

    let rssi = unsafe { rfm::rfm_getrssi() };

    Ok(radio::State {
        rx_rssi: buf[0],
        rx_voltage: buf[1],
        rx_status: buf[2],
        rx_cont1: buf[3],
        rx_cont2: buf[4],
        rx_cont3: buf[5],
        rx_cont4: buf[6],
        packet_rssi: rssi,
    })
}

/// Retrieve and parse a packet received in async receive
pub fn receive_async() -> nb::Result<radio::State, ReceiveError> {
    if unsafe { rfm::rfm_packet_waiting() } {
        let mut rx_buf: [u8; RSP_PACKET_LEN] = unsafe { mem::uninitialized() };

        if unsafe { rfm::rfm_packet_retrieve(rx_buf.as_mut_ptr(), RSP_PACKET_LEN as u8) } {
            parse_packet(rx_buf).map_err(|e| nb::Error::Other(e))
        } else {
            Err(nb::Error::Other(ReceiveError::ReceiveError))
        }
    } else {
        Err(nb::Error::WouldBlock)
    }
}
