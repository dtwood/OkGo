use bare_metal::CriticalSection;
use stm32f0xx;
use f0::adc::Adc;
use f0::dac::Dac;
use f0::out::Output;
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

adc!(BATT_MON, A, 0, 0);
adc!(RELAY_SENSE, B, 1, 9);
adc!(CONT_CH1, B, 0, 8);
adc!(CONT_CH2, A, 7, 7);
adc!(CONT_CH3, A, 6, 6);
adc!(CONT_CH4, A, 5, 5);

#[no_mangle]
pub unsafe extern "C" fn ignition_pins_init() {
    let cs = CriticalSection::new();
    init(&cs);
}

pub fn init(cs: &CriticalSection) {
    /* Clock all GPIO peripherals */
    stm32f0xx::RCC.borrow(cs).ahbenr.write(|w| {
        w.iopaen().set_bit().iopben().set_bit().iopcen().set_bit()
    });

    /* Debug LEDs.  Default off. */
    LED_GREEN.clear(cs);
    LED_YELLOW.clear(cs);
    LED_GREEN.setup(cs);
    LED_YELLOW.setup(cs);

    /* Arm/disarm LED.  Default off. */
    LED_ARM.clear(cs);
    LED_DISARM.clear(cs);
    LED_ARM.setup(cs);
    LED_DISARM.setup(cs);

    /* Upstream relay and firing channels, default all off */
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

    /* Analog inputs */
    BATT_MON.setup(cs);
    RELAY_SENSE.setup(cs);
    CONT_CH1.setup(cs);
    CONT_CH2.setup(cs);
    CONT_CH3.setup(cs);
    CONT_CH4.setup(cs);

    /* Buzzer DAC output */
    stm32f0xx::RCC
        .borrow(cs)
        .apb1enr
        .write(|w| w.dacen().set_bit());
    BUZZER.setup(cs);
    BUZZER.set_right_u8(cs, 0);
    stm32f0xx::DAC
        .borrow(cs)
        .cr
        .write(|w| w.ten1().clear_bit().en1().set_bit());
}
