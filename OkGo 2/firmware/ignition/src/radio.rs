use core::mem;
use core::slice;

use ignition;
use firmware_common::adc::adc_to_millivolts;
use bare_metal::CriticalSection;
use md_5::Md5;
use hmac::{Hmac, Mac};
use firmware_common::rfm;
use io;

extern "C" {
    /// Setup the SPI peripheral and call the RFM95W initialisation procedure.
    /// Also initialise all the above state variables to sensible defaults
    pub fn ignition_radio_init(radio_state: *mut State);

    /// Initiate packet reception and block until a packet is received
    pub fn ignition_radio_receive_blocking(radio_state: *mut State);

    /// Retrieve and parse a packet received in async receive
    pub fn ignition_radio_receive_async(radio_state: *mut State);

    /// Parse a received radio packet and fill in the received packet datastore
    pub fn ignition_radio_parse_packet(radio_state: *mut State, buf: *mut u8, len: u8);
}

/// Ignition radio state structure
#[repr(C)]
pub struct State {
    pub valid_rx: bool,
    pub lost_link: bool,
    /// RSSI of the incoming packet
    pub packet_rssi: u8,
    pub command: u8,
}

/* Convert raw ADC value to continuity ohms */
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

/// Transmit a packet to control based on the contents of state
#[no_mangle]
pub unsafe extern "C" fn ignition_radio_transmit(
    state: *mut ignition::State,
    radio_state: *const State,
) {
    let cs = CriticalSection::new();
    transmit(&cs, &mut *state, &*radio_state)
}

pub fn transmit(cs: &CriticalSection, state: &mut ignition::State, radio_state: &State) {
    let mut buf = [0; 17];

    buf[0] = radio_state.packet_rssi;

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
    let mut mac = Hmac::<Md5>::new(get_key());
    mac.input(&buf[0..7]);
    buf[7..].clone_from_slice(mac.result().code());

    unsafe {
        rfm::rfm_transmit(buf.as_ptr(), mem::size_of_val(&buf) as u8);
    }
}

fn get_key() -> &'static [u8] {
    unsafe { &slice::from_raw_parts(key, key_len.into()) }
}

extern "C" {
    static key: *const u8;
    static key_len: u8;
}
