/// Convert an ADC-read value to a voltage in millivolts
#[no_mangle]
pub extern "C" fn adc_to_millivolts(raw: u16) -> u32 {
    // From ref manual p253, V = VDD_A / FULL_SCALE * raw
    // Our VDD is always 3.3V and FULL_SCALE is 2^12 - 1
    // So V = 3.3/(2^12 - 1) * raw
    // Multiplying by 1000 for millivolts gives:
    // V*1000 = raw * 25 / 31

    u32::from(raw) * 25 / 31
}
