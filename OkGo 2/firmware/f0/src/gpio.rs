use bare_metal::CriticalSection;
use stm32f0xx;

pub enum Port {
    A,
    B,
    C,
    D,
    E,
    F,
}

pub struct Output {
    pub port: Port,
    pub pin: u32,
}

impl Output {
    pub fn set(&self, cs: &CriticalSection) {
        match self.port {
            Port::A => stm32f0xx::GPIOA.borrow(cs).odr.modify(|r, w| unsafe { w.bits(r.bits() ^ (1 << self.pin))}),
            Port::B => stm32f0xx::GPIOB.borrow(cs).odr.modify(|r, w| unsafe { w.bits(r.bits() ^ (1 << self.pin))}),
            Port::C => stm32f0xx::GPIOC.borrow(cs).odr.modify(|r, w| unsafe { w.bits(r.bits() ^ (1 << self.pin))}),
            Port::D => stm32f0xx::GPIOD.borrow(cs).odr.modify(|r, w| unsafe { w.bits(r.bits() ^ (1 << self.pin))}),
            Port::E => stm32f0xx::GPIOE.borrow(cs).odr.modify(|r, w| unsafe { w.bits(r.bits() ^ (1 << self.pin))}),
            Port::F => stm32f0xx::GPIOF.borrow(cs).odr.modify(|r, w| unsafe { w.bits(r.bits() ^ (1 << self.pin))}),
        }
    }

    pub fn clear(&self, cs: &CriticalSection) {
        match self.port {
            Port::A => stm32f0xx::GPIOA.borrow(cs).odr.modify(|r, w| unsafe { w.bits(r.bits() & !(1 << self.pin))}),
            Port::B => stm32f0xx::GPIOB.borrow(cs).odr.modify(|r, w| unsafe { w.bits(r.bits() & !(1 << self.pin))}),
            Port::C => stm32f0xx::GPIOC.borrow(cs).odr.modify(|r, w| unsafe { w.bits(r.bits() & !(1 << self.pin))}),
            Port::D => stm32f0xx::GPIOD.borrow(cs).odr.modify(|r, w| unsafe { w.bits(r.bits() & !(1 << self.pin))}),
            Port::E => stm32f0xx::GPIOE.borrow(cs).odr.modify(|r, w| unsafe { w.bits(r.bits() & !(1 << self.pin))}),
            Port::F => stm32f0xx::GPIOF.borrow(cs).odr.modify(|r, w| unsafe { w.bits(r.bits() & !(1 << self.pin))}),
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
        (match self.port {
            Port::A => stm32f0xx::GPIOA.borrow(cs).odr.read().bits() & (1 << self.pin),
            Port::B => stm32f0xx::GPIOB.borrow(cs).odr.read().bits() & (1 << self.pin),
            Port::C => stm32f0xx::GPIOC.borrow(cs).odr.read().bits() & (1 << self.pin),
            Port::D => stm32f0xx::GPIOD.borrow(cs).odr.read().bits() & (1 << self.pin),
            Port::E => stm32f0xx::GPIOE.borrow(cs).odr.read().bits() & (1 << self.pin),
            Port::F => stm32f0xx::GPIOF.borrow(cs).odr.read().bits() & (1 << self.pin),
        }) != 0
    }

    pub fn toggle(&self, cs: &CriticalSection) {
        self.set_bool(cs, !self.get_bool(cs));
    }

    pub fn setup(&self, cs: &CriticalSection) {
        match self.port {
            Port::A => {
                stm32f0xx::GPIOA.borrow(cs).moder.modify(|r, w| unsafe { w.bits(r.bits() & !(0b11 << (self.pin * 2)) | (0b01 << (self.pin * 2)))});
                stm32f0xx::GPIOA.borrow(cs).pupdr.modify(|r, w| unsafe { w.bits(r.bits() & !(0b11 << (self.pin * 2)))});
            }
            _ => panic!(),
        }
    }
}
