use bare_metal::CriticalSection;
use stm32f0xx;
use f0;

/// Good values for this are:
/// 0 - off
/// 93 - low
/// 112 - medium
/// 255 - deafening
/// The middle two might be variable or sensitive to temperature
pub fn ignition_buzzer_set(cs: &CriticalSection, value: u8) {
    let dac = stm32f0xx::DAC.borrow(cs);
    f0::dac::set_right_u8(&dac, value);
}
