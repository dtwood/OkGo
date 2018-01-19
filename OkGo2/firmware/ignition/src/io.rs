pub fn init(p: &::init::Peripherals, r: &::init::Resources) {
    // Clock all GPIO peripherals
    p.RCC.ahbenr.write(|w| {
        w
        // GPIO A
        .iopaen().set_bit()
        // GPIO B
        .iopben().set_bit()
        // GPIO C
        .iopcen().set_bit()
    });
    p.RCC.apb1enr.write(|w| {
        w
        // DAC
        .dacen().set_bit()
    });
    p.RCC.apb2enr.write(|w| {
        w
        // ADC
        .adcen().set_bit()
    });

    // Analog inputs
    // f0::adc::set_up_adc();
    r.BATT_MON.setup();
    r.RELAY_SENSE.setup();
    r.CONT_CH1.setup();
    r.CONT_CH2.setup();
    r.CONT_CH3.setup();
    r.CONT_CH4.setup();
}
