use stm32f0xx;
use f0::output::OutputT;
use f0::gpio::GpioT;

pub trait Led {
    type GPIO;
    type RCC;

    fn init(&self, gpio: &Self::GPIO, rcc: &Self::RCC);
    fn on(&self, gpio: &Self::GPIO);
    fn off(&self, gpio: &Self::GPIO);
    fn toggle(&self, gpio: &Self::GPIO);
    fn set_bool(&self, gpio: &Self::GPIO, value: bool);
}

pub struct LedB(u32);
impl Led for LedB {
    type GPIO = stm32f0xx::GPIOB;
    type RCC = stm32f0xx::RCC;

    fn init(&self, gpio: &Self::GPIO, rcc: &Self::RCC) {
        gpio.init(rcc);
        gpio.set_output(self.0);
    }
    fn on(&self, gpio: &Self::GPIO) {
        gpio.high(self.0)
    }
    fn off(&self, gpio: &Self::GPIO) {
        gpio.low(self.0)
    }
    fn toggle(&self, gpio: &Self::GPIO) {
        gpio.toggle(self.0);
    }
    fn set_bool(&self, gpio: &Self::GPIO, value: bool) {
        gpio.set_bool(self.0, value);
    }
}

pub struct RelayA(u32);
pub struct RelayB(u32);

pub trait Relay {
    type GPIO: OutputT + GpioT;

    fn pin(&self) -> u32;

    fn init(&self, gpio: &Self::GPIO, rcc: &stm32f0xx::RCC) {
        gpio.init(rcc);
        gpio.set_output(self.pin());
    }
    fn close(&self, gpio: &Self::GPIO) {
        gpio.high(self.pin())
    }
    fn open(&self, gpio: &Self::GPIO) {
        gpio.low(self.pin())
    }
    fn set_bool(&self, gpio: &Self::GPIO, value: bool) {
        gpio.set_bool(self.pin(), value);
    }
}

impl Relay for RelayA {
    type GPIO = stm32f0xx::GPIOA;

    fn pin(&self) -> u32 {
        self.0
    }
}

impl Relay for RelayB {
    type GPIO = stm32f0xx::GPIOB;

    fn pin(&self) -> u32 {
        self.0
    }
}

pub mod relays {
    use super::*;

    pub static UPSTREAM: RelayA = RelayA(10);
    pub static CHANNEL_1: RelayA = RelayA(9);
    pub static CHANNEL_2: RelayA = RelayA(8);
    pub static CHANNEL_3: RelayB = RelayB(15);
    pub static CHANNEL_4: RelayB = RelayB(14);
}

pub mod leds {
    use super::*;

    pub static GREEN: LedB = LedB(13);
    pub static YELLOW: LedB = LedB(12);
    pub static ARM: LedB = LedB(8);
    pub static DISARM: LedB = LedB(9);
}
