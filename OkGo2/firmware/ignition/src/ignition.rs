use radio;
use bare_metal::CriticalSection;
use io;
use libopencm3_sys;

#[repr(C)]
pub struct State {
    pub armed: bool,
    pub fire_ch1: bool,
    pub fire_ch2: bool,
    pub fire_ch3: bool,
    pub fire_ch4: bool,
    pub beep_start: u32,
    pub beep_volume: u8,
    pub packet_rssi: u8,
}

pub fn init(cs: &CriticalSection) {
    // Setup crystal oscillator and systick
    unsafe {
        libopencm3_sys::rcc_clock_setup_in_hsi_out_48mhz();
        libopencm3_sys::systick_init();
    }

    // Clock GPIOs, set pin modes
    io::init(cs);

    // Initialise radio and local state variables, read stored config
    radio::init(cs);
}
