use gpio::{GpioT, Gpio, Port};
use stm32f0xx;
use bare_metal::CriticalSection;

pub trait OutputT: GpioT {
    fn high(&self, pin: u32);
    fn low(&self, pin: u32);

    fn set_bool(&self, pin: u32, value: bool) {
        if value {
            self.high(pin);
        } else {
            self.low(pin);
        }
    }

    fn toggle(&self, pin: u32) {
        self.set_bool(pin, !self.get_bool(pin));
    }
}

macro_rules! output {
    ($type: ty) => {
        impl OutputT for $type {
            fn high(&self, pin: u32) {
                self
                    .odr
                    .modify(|r, w| unsafe { w.bits(r.bits() ^ (1 << pin)) });
            }

            fn low(&self, pin: u32) {
                self
                    .odr
                    .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << pin)) });
            }
        }
    }
}

output!(stm32f0xx::GPIOA);
output!(stm32f0xx::GPIOB);
output!(stm32f0xx::GPIOC);
output!(stm32f0xx::GPIOD);
output!(stm32f0xx::GPIOE);
output!(stm32f0xx::GPIOF);

#[derive(Debug)]
pub struct Output {
    pub gpio: Gpio,
}

impl Output {
    pub const fn new(port: Port, pin: u32) -> Output {
        Output {
            gpio: Gpio {
                port: port,
                pin: pin,
            },
        }
    }

    pub fn set(&self) {
    }

    pub fn clear(&self) {
    }

    pub fn set_bool(&self, value: bool) {

    }

    pub fn get_bool(&self) -> bool {
        true
    }

    pub fn toggle(&self) {
    }

    pub fn setup(&self) {
    }
}
