use gpio::{Gpio, Port, Mode, PullUpDown};
use stm32f0xx;
use bare_metal::CriticalSection;

#[macro_export]
macro_rules! output {
    ($id: ident, $port: ident, $pin: expr) => {
        pub static $id: Output = Output {
            gpio: Gpio {
                port: Port::$port,
                pin: $pin,
            },
        };
    }
}

#[derive(Debug)]
pub struct Output {
    pub gpio: Gpio,
}

impl Output {
    pub fn set(&self, cs: &CriticalSection) {
        match self.gpio.port {
            Port::A => stm32f0xx::GPIOA.borrow(cs).odr.modify(|r, w| unsafe { w.bits(r.bits() ^ (1 << self.gpio.pin))}),
            Port::B => stm32f0xx::GPIOB.borrow(cs).odr.modify(|r, w| unsafe { w.bits(r.bits() ^ (1 << self.gpio.pin))}),
            Port::C => stm32f0xx::GPIOC.borrow(cs).odr.modify(|r, w| unsafe { w.bits(r.bits() ^ (1 << self.gpio.pin))}),
            Port::D => stm32f0xx::GPIOD.borrow(cs).odr.modify(|r, w| unsafe { w.bits(r.bits() ^ (1 << self.gpio.pin))}),
            Port::E => stm32f0xx::GPIOE.borrow(cs).odr.modify(|r, w| unsafe { w.bits(r.bits() ^ (1 << self.gpio.pin))}),
            Port::F => stm32f0xx::GPIOF.borrow(cs).odr.modify(|r, w| unsafe { w.bits(r.bits() ^ (1 << self.gpio.pin))}),
        }
    }

    pub fn clear(&self, cs: &CriticalSection) {
        match self.gpio.port {
            Port::A => stm32f0xx::GPIOA.borrow(cs).odr.modify(|r, w| unsafe { w.bits(r.bits() & !(1 << self.gpio.pin))}),
            Port::B => stm32f0xx::GPIOB.borrow(cs).odr.modify(|r, w| unsafe { w.bits(r.bits() & !(1 << self.gpio.pin))}),
            Port::C => stm32f0xx::GPIOC.borrow(cs).odr.modify(|r, w| unsafe { w.bits(r.bits() & !(1 << self.gpio.pin))}),
            Port::D => stm32f0xx::GPIOD.borrow(cs).odr.modify(|r, w| unsafe { w.bits(r.bits() & !(1 << self.gpio.pin))}),
            Port::E => stm32f0xx::GPIOE.borrow(cs).odr.modify(|r, w| unsafe { w.bits(r.bits() & !(1 << self.gpio.pin))}),
            Port::F => stm32f0xx::GPIOF.borrow(cs).odr.modify(|r, w| unsafe { w.bits(r.bits() & !(1 << self.gpio.pin))}),
        }
    }

    pub fn set_bool(&self, cs: &CriticalSection, value: bool) {
        if value {
            self.set(cs);
        } else {
            self.clear(cs);
        }
    }

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

    pub fn toggle(&self, cs: &CriticalSection) {
        self.set_bool(cs, !self.get_bool(cs));
    }

    pub fn setup(&self, cs: &CriticalSection) {
        self.gpio.setup(cs, Mode::Output, PullUpDown::None);
    }
}
