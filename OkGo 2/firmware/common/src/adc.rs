use stm32f0xx;
use bare_metal::CriticalSection;

/// General purpose ADC initialisation and calibration
#[no_mangle]
pub unsafe extern "C" fn adc_init() {
    let cs = CriticalSection::new();
    let adc = stm32f0xx::ADC.borrow(&cs);

    adc.cr.write(|w| w.addis().set_bit());
    while adc.cr.read().aden().bit_is_set() {
        /* Wait for the ADC to be disabled */
    }

    adc.cfgr2.reset();

    adc.cr.write(|w| w.adcal().set_bit());
    while adc.cr.read().adcal().bit_is_set() {
        /* Wait for calibration to finish */
    }

    adc.cfgr1.write(|w| w
        .cont().clear_bit()
        .discen().set_bit()
        .align().set_bit()
        .res().bits(0)
    );

    adc.smpr.write(|w| w
        .smpr().bits(6)
    );

    adc.cr.write(|w| w.aden().set_bit());
    while adc.cr.read().aden().bit_is_clear() {
        /* Wait for the ADC to be enabled */
    }
}

// uint16_t adc_read(uint8_t channel)
// {
//     /* Set appropriate channel in CHSEL */
//     adc_set_regular_sequence(ADC1, 1, &channel);
//
//     /* Start conversion and block until completion */
//     adc_start_conversion_regular(ADC1);
//     while(!adc_eoc(ADC1));
//
//     return adc_read_regular(ADC1);
// }

/// Read an ADC value, blocking and returning result
#[no_mangle]
pub unsafe extern "C" fn adc_read(channel: u8) -> u16 {
    let cs = CriticalSection::new();
    let adc = stm32f0xx::ADC.borrow(&cs);

    assert!(channel <= 18);

    adc.chselr.write(|w| w
        .bits(1 << channel)
    );

    adc.cr.write(|w| w
        .adstart().set_bit()
    );
    while adc.isr.read().eoc().bit_is_clear() {
        /* Wait for the conversion to finish */
    }

    adc.dr.read().data().bits()
}

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
