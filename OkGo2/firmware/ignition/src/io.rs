use bare_metal::CriticalSection;
use stm32f0xx;
use f0::dac::Dac;
use f0::gpio::{Gpio, Port};

/// Good values for this are:
/// 0 - off
/// 93 - low
/// 112 - medium
/// 255 - deafening
/// The middle two might be variable or sensitive to temperature
pub fn ignition_buzzer_set(cs: &CriticalSection, value: u8) {
    BUZZER.set_right_u8(cs, value);
}

output!(LED_GREEN, B, 13);
output!(LED_YELLOW, B, 12);
output!(LED_ARM, B, 8);
output!(LED_DISARM, B, 9);

output!(UPSTREAM_RELAY, A, 10);
output!(FIRE_CH1, A, 9);
output!(FIRE_CH2, A, 8);
output!(FIRE_CH3, B, 15);
output!(FIRE_CH4, B, 14);

dac!(BUZZER, A, 4);

adc!(ADC);
adc_pin!(BATT_MON, ADC, A, 0, 0);
adc_pin!(RELAY_SENSE, ADC, B, 1, 9);
adc_pin!(CONT_CH1, ADC, B, 0, 8);
adc_pin!(CONT_CH2, ADC, A, 7, 7);
adc_pin!(CONT_CH3, ADC, A, 6, 6);
adc_pin!(CONT_CH4, ADC, A, 5, 5);

pub fn init(cs: &CriticalSection) {
    // Clock all GPIO peripherals
    let rcc = stm32f0xx::RCC.borrow(cs);
    rcc.ahbenr.write(|w| {
        w
        // GPIO A
        .iopaen().set_bit()
        // GPIO B
        .iopben().set_bit()
        // GPIO C
        .iopcen().set_bit()
    });
    rcc.apb1enr.write(|w| {
        w
        // DAC
        .dacen().set_bit()
    });
    rcc.apb2enr.write(|w| {
        w
        // ADC
        .adcen().set_bit()
    });

    // Debug LEDs.  Default off.
    LED_GREEN.clear(cs);
    LED_YELLOW.clear(cs);
    LED_GREEN.setup(cs);
    LED_YELLOW.setup(cs);

    // Arm/disarm LED.  Default off.
    LED_ARM.clear(cs);
    LED_DISARM.clear(cs);
    LED_ARM.setup(cs);
    LED_DISARM.setup(cs);

    // Upstream relay and firing channels, default all off
    UPSTREAM_RELAY.clear(cs);
    FIRE_CH1.clear(cs);
    FIRE_CH2.clear(cs);
    FIRE_CH3.clear(cs);
    FIRE_CH4.clear(cs);
    UPSTREAM_RELAY.setup(cs);
    FIRE_CH1.setup(cs);
    FIRE_CH2.setup(cs);
    FIRE_CH3.setup(cs);
    FIRE_CH4.setup(cs);

    // Analog inputs
    ADC.setup(cs);
    BATT_MON.setup(cs);
    RELAY_SENSE.setup(cs);
    CONT_CH1.setup(cs);
    CONT_CH2.setup(cs);
    CONT_CH3.setup(cs);
    CONT_CH4.setup(cs);

    // Buzzer DAC output
    BUZZER.setup(cs);
    BUZZER.set_right_u8(cs, 0);
    stm32f0xx::DAC
        .borrow(cs)
        .cr
        .write(|w| w.ten1().clear_bit().en1().set_bit());
}
