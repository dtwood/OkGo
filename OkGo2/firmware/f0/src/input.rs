use gpio::{Gpio, Mode, Port, PullUpDown};
use stm32f0xx;
use bare_metal::CriticalSection;

#[derive(Debug)]
pub struct Input {
    pub gpio: Gpio,
}

impl Input {
    pub const fn new(port: Port, pin: u32) -> Input {
        Input {
            gpio: Gpio {
                port: port,
                pin: pin,
            },
        }
    }

    pub fn get_bool(&self) -> bool {
        let cs = unsafe { CriticalSection::new() };
        let cs = &cs;

        (match self.gpio.port {
            Port::A => stm32f0xx::GPIOA.borrow(cs).odr.read().bits() & (1 << self.gpio.pin),
            Port::B => stm32f0xx::GPIOB.borrow(cs).odr.read().bits() & (1 << self.gpio.pin),
            Port::C => stm32f0xx::GPIOC.borrow(cs).odr.read().bits() & (1 << self.gpio.pin),
            Port::D => stm32f0xx::GPIOD.borrow(cs).odr.read().bits() & (1 << self.gpio.pin),
            Port::E => stm32f0xx::GPIOE.borrow(cs).odr.read().bits() & (1 << self.gpio.pin),
            Port::F => stm32f0xx::GPIOF.borrow(cs).odr.read().bits() & (1 << self.gpio.pin),
        }) != 0
    }

    pub fn setup(&self, pupd: PullUpDown) {
        self.gpio.setup(Mode::Input, pupd);
    }
}
