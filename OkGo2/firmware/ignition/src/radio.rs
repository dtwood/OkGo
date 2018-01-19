use core::mem;

use md5::Md5;
use hmac::{Hmac, Mac};
use firmware_common::key;
use nb;
use rfm95w::Rfm95w;
use embedded_hal::blocking::spi;
use embedded_hal::digital::OutputPin;

pub const REQ_PACKET_LEN: u8 = 11;
const CFM_PACKET_LEN: u8 = 17;

/// Radio tx power in dBm
pub const RADIO_POWER_DBM: u8 = 10;

pub struct ReqPacket {
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

fn parse_packet(buf: [u8; REQ_PACKET_LEN as usize]) -> Result<ReqPacket, ReceiveError> {
    let mut mac = Hmac::<Md5>::new(&key::KEY).unwrap();
    mac.input(&buf[0..(REQ_PACKET_LEN as usize - 10)]);
    if mac.verify(&buf[(REQ_PACKET_LEN as usize - 10)..]).is_err() {
        return Err(ReceiveError::InvalidHash);
    }

    let command = buf[0];

    let armed = command & (1 << 4) != 0;
    let beep_volume = (command >> 5) & 0x07;

    Ok(if armed {
        ReqPacket {
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

pub trait RadioExt {
    /// Transmit a packet to control based on the contents of state
    fn transmit_packet(&mut self, packet: &CfmPacket);

    /// Retrieve and parse a packet received in async receive
    fn receive_packet(&mut self) -> nb::Result<(u8, ReqPacket), ReceiveError>;
}

impl<SPI, NSS, E> RadioExt for Rfm95w<SPI, NSS>
where
    SPI: spi::Transfer<u8, Error = E> + spi::Write<u8, Error = E> + Send,
    NSS: OutputPin + Send,
    E: ::core::fmt::Debug,
{
    /// Transmit a packet to control based on the contents of state
    fn transmit_packet(&mut self, packet: &CfmPacket) {
        let mut buf = [0; CFM_PACKET_LEN as usize];

        buf[0] = packet.received_rssi;
        buf[1] = packet.battery_voltage;

        buf[2] = ((packet.armed as u8) << 4) | ((packet.fire_ch4 as u8) << 3)
            | ((packet.fire_ch3 as u8) << 2) | ((packet.fire_ch2 as u8) << 1)
            | (packet.fire_ch1 as u8);

        buf[3] = packet.cont_ch1;
        buf[4] = packet.cont_ch2;
        buf[5] = packet.cont_ch3;
        buf[6] = packet.cont_ch4;

        /* Generate message HMAC signature */
        let mut mac = Hmac::<Md5>::new(&key::KEY).unwrap();
        mac.input(&buf[0..(CFM_PACKET_LEN as usize - 10)]);
        buf[(CFM_PACKET_LEN as usize - 10)..].clone_from_slice(&mac.result().code());

        self.transmit(&buf).unwrap();
    }

    /// Retrieve and parse a packet received in async receive
    fn receive_packet(&mut self) -> nb::Result<(u8, ReqPacket), ReceiveError> {
        if self.packet_waiting().unwrap() {
            let mut rx_buf: [u8; REQ_PACKET_LEN as usize] = unsafe { mem::uninitialized() };

            if self.packet_retrieve(&mut rx_buf).unwrap() {
                let rssi = self.getrssi().unwrap();
                parse_packet(rx_buf)
                    .map(|p| (rssi, p))
                    .map_err(nb::Error::Other)
            } else {
                Err(nb::Error::Other(ReceiveError::ReceiveError))
            }
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

pub enum ReceiveError {
    ReceiveError,
    InvalidHash,
}
