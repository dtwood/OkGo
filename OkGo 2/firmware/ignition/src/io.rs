use bare_metal::CriticalSection;
use stm32f0xx;
use f0::adc::Adc;
use f0::dac::Dac;
use f0::gpio::{Port, Output};

/// Good values for this are:
/// 0 - off
/// 93 - low
/// 112 - medium
/// 255 - deafening
/// The middle two might be variable or sensitive to temperature
pub fn ignition_buzzer_set(cs: &CriticalSection, value: u8) {
    BUZZER.set_right_u8(cs, value);
}

pub static LED_GREEN: Output = Output {
    port: Port::B,
    pin: 13,
};
pub static LED_YELLOW: Output = Output {
    port: Port::B,
    pin: 12,
};

pub static LED_ARM: Output = Output {
    port: Port::B,
    pin: 8,
};
pub static LED_DISARM: Output = Output {
    port: Port::B,
    pin: 9,
};

pub static UPSTREAM_RELAY: Output = Output {
    port: Port::A,
    pin: 10,
};
pub static FIRE_CH1: Output = Output {
    port: Port::A,
    pin: 9,
};
pub static FIRE_CH2: Output = Output {
    port: Port::A,
    pin: 8,
};
pub static FIRE_CH3: Output = Output {
    port: Port::B,
    pin: 15,
};
pub static FIRE_CH4: Output = Output {
    port: Port::B,
    pin: 14,
};

pub static BUZZER: Dac = Dac {
    port: Port::A,
    pin: 4,
};

pub static BATT_MON: Adc = Adc {
    port: Port::A,
    pin: 0,
    channel: 0,
};
pub static RELAY_SENSE: Adc = Adc {
    port: Port::B,
    pin: 1,
    channel: 9,
};
pub static CONT_CH1: Adc = Adc {
    port: Port::B,
    pin: 0,
    channel: 8,
};
pub static CONT_CH2: Adc = Adc {
    port: Port::A,
    pin: 7,
    channel: 7,
};
pub static CONT_CH3: Adc = Adc {
    port: Port::A,
    pin: 6,
    channel: 6,
};
pub static CONT_CH4: Adc = Adc {
    port: Port::A,
    pin: 5,
    channel: 5,
};

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

    BATT_MON.setup(cs);
    RELAY_SENSE.setup(cs);
    CONT_CH1.setup(cs);
    CONT_CH2.setup(cs);
    CONT_CH3.setup(cs);
    CONT_CH4.setup(cs);

    // /* Analog inputs */
    // gpio_mode_setup(BATT_MON_PORT, GPIO_MODE_ANALOG, GPIO_PUPD_NONE, BATT_MON);
    // gpio_mode_setup(RELAY_SENSE_PORT, GPIO_MODE_ANALOG, GPIO_PUPD_NONE,
    //                 RELAY_SENSE);
    // gpio_mode_setup(CONT_CH1_PORT, GPIO_MODE_ANALOG, GPIO_PUPD_NONE, CONT_CH1);
    // gpio_mode_setup(CONT_CH2_PORT, GPIO_MODE_ANALOG, GPIO_PUPD_NONE, CONT_CH2);
    // gpio_mode_setup(CONT_CH3_PORT, GPIO_MODE_ANALOG, GPIO_PUPD_NONE, CONT_CH3);
    // gpio_mode_setup(CONT_CH4_PORT, GPIO_MODE_ANALOG, GPIO_PUPD_NONE, CONT_CH4);

    /* Buzzer DAC output */
    stm32f0xx::RCC.borrow(cs).apb1enr.write(|w| w.dacen().set_bit());
    BUZZER.setup(cs);
    BUZZER.set_right_u8(cs, 0);
    stm32f0xx::DAC.borrow(cs).cr.write(|w| w.ten1().clear_bit().en1().set_bit());
}
