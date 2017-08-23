use gpio::{Gpio, Mode, Port, PullUpDown};
use stm32f0xx;
use bare_metal::CriticalSection;

#[macro_export]
macro_rules! output {
    ($id: ident, $port: ident, $pin: expr) => {
        pub static $id: $crate::output::Output = $crate::output::Output {
            gpio: $crate::gpio::Gpio {
                port: $crate::gpio::Port::$port,
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
    pub const fn new(port: Port, pin: u32) -> Output {
        Output {
            gpio: Gpio {
                port: port,
                pin: pin,
            },
        }
    }

    pub fn set(&self) {
        let cs = unsafe { CriticalSection::new() };
        let cs = &cs;
        match self.gpio.port {
            Port::A => stm32f0xx::GPIOA
                .borrow(cs)
                .odr
                .modify(|r, w| unsafe { w.bits(r.bits() ^ (1 << self.gpio.pin)) }),
            Port::B => stm32f0xx::GPIOB
                .borrow(cs)
                .odr
                .modify(|r, w| unsafe { w.bits(r.bits() ^ (1 << self.gpio.pin)) }),
            Port::C => stm32f0xx::GPIOC
                .borrow(cs)
                .odr
                .modify(|r, w| unsafe { w.bits(r.bits() ^ (1 << self.gpio.pin)) }),
            Port::D => stm32f0xx::GPIOD
                .borrow(cs)
                .odr
                .modify(|r, w| unsafe { w.bits(r.bits() ^ (1 << self.gpio.pin)) }),
            Port::E => stm32f0xx::GPIOE
                .borrow(cs)
                .odr
                .modify(|r, w| unsafe { w.bits(r.bits() ^ (1 << self.gpio.pin)) }),
            Port::F => stm32f0xx::GPIOF
                .borrow(cs)
                .odr
                .modify(|r, w| unsafe { w.bits(r.bits() ^ (1 << self.gpio.pin)) }),
        }
    }

    pub fn clear(&self) {
        let cs = unsafe { CriticalSection::new() };
        let cs = &cs;
        match self.gpio.port {
            Port::A => stm32f0xx::GPIOA
                .borrow(cs)
                .odr
                .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << self.gpio.pin)) }),
            Port::B => stm32f0xx::GPIOB
                .borrow(cs)
                .odr
                .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << self.gpio.pin)) }),
            Port::C => stm32f0xx::GPIOC
                .borrow(cs)
                .odr
                .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << self.gpio.pin)) }),
            Port::D => stm32f0xx::GPIOD
                .borrow(cs)
                .odr
                .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << self.gpio.pin)) }),
            Port::E => stm32f0xx::GPIOE
                .borrow(cs)
                .odr
                .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << self.gpio.pin)) }),
            Port::F => stm32f0xx::GPIOF
                .borrow(cs)
                .odr
                .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << self.gpio.pin)) }),
        }
    }

    pub fn set_bool(&self, value: bool) {
        if value {
            self.set();
        } else {
            self.clear();
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

    pub fn toggle(&self) {
        self.set_bool(!self.get_bool());
    }

    pub fn setup(&self) {
        self.gpio.setup(Mode::Output, PullUpDown::None);
    }
}
