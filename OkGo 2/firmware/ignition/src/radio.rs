use core::mem;
use core::slice;

use ignition;
use firmware_common::adc::adc_to_millivolts;
use f0::adc;
use bare_metal::CriticalSection;
use stm32f0xx;
use md_5::Md5;
use hmac::{Hmac, Mac};

static ADC_CH_IGTN_BATT: u8 = 0;
static ADC_CH_IGTN_CONT1: u8 = 8;
static ADC_CH_IGTN_CONT2: u8 = 7;
static ADC_CH_IGTN_CONT3: u8 = 6;
static ADC_CH_IGTN_CONT4: u8 = 5;

/// Ignition radio state structure
#[repr(C)]
pub struct State {
    valid_rx: bool,
    lost_link: bool,
    /// RSSI of the incoming packet
    packet_rssi: u8,
    command: u8,
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
    radio_state: *mut State
) {
    let cs = CriticalSection::new();
    transmit(&cs, &mut *state, &mut *radio_state)
}

fn transmit(cs: &CriticalSection, state: &mut ignition::State, radio_state: &mut State)
{
    let adc = stm32f0xx::ADC.borrow(&cs);

    let mut buf = [0; 17];

    buf[0] = radio_state.packet_rssi;

    let mut adc_val: u32 = adc_to_millivolts(adc::read(adc, ADC_CH_IGTN_BATT));

    adc_val = adc_val * 133 / 33;
    if (adc_val / 10) % 10 >= 5 /* Round instead of truncate */
    {
        buf[1] = (adc_val / 100 + 1) as u8;
    } else {
        buf[1] = (adc_val / 100) as u8;
    }
    let status: u8 = ((state.armed as u8) << 4) |
             ((state.fire_ch4 as u8) << 3) |
             ((state.fire_ch3 as u8) << 2) |
             ((state.fire_ch2 as u8) << 1) |
             (state.fire_ch1 as u8);
    buf[2] = status;

    buf[3] = adc_to_ohms(adc::read(adc, ADC_CH_IGTN_CONT1));
    buf[4] = adc_to_ohms(adc::read(adc, ADC_CH_IGTN_CONT2));
    buf[5] = adc_to_ohms(adc::read(adc, ADC_CH_IGTN_CONT3));
    buf[6] = adc_to_ohms(adc::read(adc, ADC_CH_IGTN_CONT4));

    /* Generate message HMAC signature */
    let mut mac = Hmac::<Md5>::new(get_key());
    mac.input(&buf[0..7]);
    buf[7..].clone_from_slice(mac.result().code());

    unsafe {
        rfm_transmit(buf.as_ptr(), mem::size_of_val(&buf) as u8);
    }
}

fn get_key() -> &'static [u8] {
    unsafe { &slice::from_raw_parts(key, key_len.into()) }
}

extern {
    static key: *const u8;
    static key_len: u8;

    fn rfm_transmit(buf: *const u8, buf_len: u8);
}

// void ignition_radio_transmit(ignition_state *state,
//                              ignition_radio_state *radio_state)
// {
//     uint8_t buf[17];
//     uint32_t adc_val;
//     uint8_t status;
//
//     buf[0] = radio_state->packet_rssi;
//
//     /* Read battery voltage */
//     adc_val = adc_to_millivolts(adc_read(ADC_CH_IGTN_BATT));
//     /* Pot divider of 10k over 3k3 s.t. V = Vbatt * 3.3/13.3
//      * Vbatt = V * 13.3/3.3 = V * 133/33 */
//     adc_val = adc_val * 133 / 33;
//     if((adc_val / 10) % 10 >= 5) /* Round instead of truncate */
//         buf[1] = adc_val / 100 + 1;
//     else
//         buf[1] = adc_val / 100;
//
//     /* Status byte */
//     status = (state->armed << 4) |
//              (state->fire_ch4 << 3) |
//              (state->fire_ch3 << 2) |
//              (state->fire_ch2 << 1) |
//              (state->fire_ch1);
//     buf[2] = status;
//
//     /* Channel continuities */
//     buf[3] = adc_to_ohms(adc_read(ADC_CH_IGTN_CONT1));
//     buf[4] = adc_to_ohms(adc_read(ADC_CH_IGTN_CONT2));
//     buf[5] = adc_to_ohms(adc_read(ADC_CH_IGTN_CONT3));
//     buf[6] = adc_to_ohms(adc_read(ADC_CH_IGTN_CONT4));
//
//     /* Generate message HMAC signature */
//     hmac_md5_80(buf, 7, key, key_len, buf + 7);
//
//     rfm_transmit(buf, 17);
// }
