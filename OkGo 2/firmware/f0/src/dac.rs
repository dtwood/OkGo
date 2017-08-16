use stm32f0xx;

/// Read an ADC value, blocking and returning result
pub fn set_right_u8(dac: &stm32f0xx::DAC, value: u8) {
    dac.dhr8r1.write(|w| unsafe { w.dacc1dhr().bits(value) });
}
