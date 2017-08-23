use f0;

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

    // Debug LEDs.  Default off.
    r.LED_GREEN.clear();
    r.LED_YELLOW.clear();
    r.LED_GREEN.setup();
    r.LED_YELLOW.setup();

    // Arm/disarm LED.  Default off.
    r.LED_ARM.clear();
    r.LED_DISARM.clear();
    r.LED_ARM.setup();
    r.LED_DISARM.setup();

    // Upstream relay and firing channels, default all off
    r.UPSTREAM_RELAY.clear();
    r.FIRE_CH1.clear();
    r.FIRE_CH2.clear();
    r.FIRE_CH3.clear();
    r.FIRE_CH4.clear();
    r.UPSTREAM_RELAY.setup();
    r.FIRE_CH1.setup();
    r.FIRE_CH2.setup();
    r.FIRE_CH3.setup();
    r.FIRE_CH4.setup();

    // Analog inputs
    f0::adc::set_up_adc();
    r.BATT_MON.setup();
    r.RELAY_SENSE.setup();
    r.CONT_CH1.setup();
    r.CONT_CH2.setup();
    r.CONT_CH3.setup();
    r.CONT_CH4.setup();

    // Buzzer DAC output
    r.BEEPER.setup();
    p.DAC.cr.write(|w| w.ten1().clear_bit().en1().set_bit());
}
