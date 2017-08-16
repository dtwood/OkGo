use radio;
use firmware_common::rfm;
use bare_metal::CriticalSection;
use f0::adc;
use io;
use libopencm3_sys;

#[repr(C)]
pub struct State {
    pub armed: bool,
    pub fire_ch1: bool,
    pub fire_ch2: bool,
    pub fire_ch3: bool,
    pub fire_ch4: bool,
    pub centre_frf: u32,
    pub beep_start: u32,
    pub beep_volume: u8,
}

/// Radio tx power in dBm
const RADIO_POWER_DBM: u8 = 10;

pub fn init(cs: &CriticalSection, state: &mut State, radio_state: &mut radio::State)
{
    // Initialise local state variables
    state.armed = false;
    state.fire_ch1 = false;
    state.fire_ch2 = false;
    state.fire_ch3 = false;
    state.fire_ch4 = false;
    state.centre_frf = rfm::Frf::Frf868 as u32;
    state.beep_start = 0;
    state.beep_volume = 2;

    // Setup crystal oscillator and systick
    unsafe {
        libopencm3_sys::rcc_clock_setup_in_hsi_out_48mhz();
        libopencm3_sys::systick_init();
    }

    /* Clock GPIOs, set pin modes */
    io::init(cs);

    // Initialise radio and local state variables, read stored config
    radio::init(cs, radio_state);
    unsafe {
        rfm::rfm_setfreq(state.centre_frf);
        rfm::rfm_setpower(RADIO_POWER_DBM);
    }

    /* ADC Setup: Clock periph, run init. Pins done in ignition_pins */
    // rcc_periph_clock_enable(RCC_ADC);
    adc::init(cs);
}
