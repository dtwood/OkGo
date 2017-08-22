use gpio::{Gpio, Mode, Port, PullUpDown};
use stm32f0xx;
use bare_metal::CriticalSection;

#[macro_export]
macro_rules! input {
    ($id: ident, $port: ident, $pin: expr) => {
        pub static $id: $crate::input::Input = $crate::input::Input {
            gpio: $crate::gpio::Gpio {
                port: $crate::gpio::Port::$port,
                pin: $pin,
            },
        };
    }
}

#[derive(Debug)]
pub struct Input {
    pub gpio: Gpio,
}

impl Input {
    pub fn get_bool(&self, cs: &CriticalSection) -> bool {
        (match self.gpio.port {
            Port::A => stm32f0xx::GPIOA.borrow(cs).odr.read().bits() & (1 << self.gpio.pin),
            Port::B => stm32f0xx::GPIOB.borrow(cs).odr.read().bits() & (1 << self.gpio.pin),
            Port::C => stm32f0xx::GPIOC.borrow(cs).odr.read().bits() & (1 << self.gpio.pin),
            Port::D => stm32f0xx::GPIOD.borrow(cs).odr.read().bits() & (1 << self.gpio.pin),
            Port::E => stm32f0xx::GPIOE.borrow(cs).odr.read().bits() & (1 << self.gpio.pin),
            Port::F => stm32f0xx::GPIOF.borrow(cs).odr.read().bits() & (1 << self.gpio.pin),
        }) != 0
    }

    pub fn setup(&self, cs: &CriticalSection, pupd: PullUpDown) {
        self.gpio.setup(cs, Mode::Input, pupd);
    }
}
