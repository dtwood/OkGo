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
