use bare_metal::CriticalSection;
use stm32f0xx;
use f0;
use f0::gpio::{Port, Output};

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

pub static LED_GREEN: Output = Output { port: Port::B, pin: 13 };
pub static LED_YELLOW: Output = Output { port: Port::B, pin: 12 };

pub static LED_ARM: Output = Output { port: Port::B, pin: 8 };
pub static LED_DISARM: Output = Output { port: Port::B, pin: 9 };

pub static UPSTREAM_RELAY: Output = Output { port: Port::A, pin: 10 };
pub static FIRE_CH1: Output = Output { port: Port::A, pin: 9 };
pub static FIRE_CH2: Output = Output { port: Port::A, pin: 8 };
pub static FIRE_CH3: Output = Output { port: Port::B, pin: 15 };
pub static FIRE_CH4: Output = Output { port: Port::B, pin: 14 };

#[no_mangle]
pub unsafe extern "C" fn ignition_pins_init() {
    let cs = CriticalSection::new();
    init(&cs);
}

pub fn init(cs: &CriticalSection) {
    /* Clock all GPIO peripherals */
    stm32f0xx::RCC.borrow(cs).ahbenr.write(|w| w
        .iopaen().set_bit()
        .iopben().set_bit()
        .iopcen().set_bit()
    );

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

    // /* Analog inputs */
    // gpio_mode_setup(BATT_MON_PORT, GPIO_MODE_ANALOG, GPIO_PUPD_NONE, BATT_MON);
    // gpio_mode_setup(RELAY_SENSE_PORT, GPIO_MODE_ANALOG, GPIO_PUPD_NONE,
    //                 RELAY_SENSE);
    // gpio_mode_setup(CONT_CH1_PORT, GPIO_MODE_ANALOG, GPIO_PUPD_NONE, CONT_CH1);
    // gpio_mode_setup(CONT_CH2_PORT, GPIO_MODE_ANALOG, GPIO_PUPD_NONE, CONT_CH2);
    // gpio_mode_setup(CONT_CH3_PORT, GPIO_MODE_ANALOG, GPIO_PUPD_NONE, CONT_CH3);
    // gpio_mode_setup(CONT_CH4_PORT, GPIO_MODE_ANALOG, GPIO_PUPD_NONE, CONT_CH4);
    //
    // /* Buzzer DAC output */
    // /* Clock the DAC: */
    // rcc_periph_clock_enable(RCC_DAC);
    // /* Set GPIO to analog to disable the digital stuff attached */
    // gpio_mode_setup(BUZZER_PORT, GPIO_MODE_ANALOG, GPIO_PUPD_NONE, BUZZER);
    // /* Now setup the DAC.  Load in 0 before enabling to turn off buzzer */
    // dac_load_data_buffer_single(0, RIGHT8, CHANNEL_1);
    // dac_trigger_disable(CHANNEL_1);
    // dac_enable(CHANNEL_1);
}
