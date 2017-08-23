use bare_metal::CriticalSection;
use stm32f0xx;

#[derive(Copy, Clone, Debug)]
pub enum Port {
    A,
    B,
    C,
    D,
    E,
    F,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum Mode {
    Input = 0b00,
    Output = 0b01,
    Analog = 0b11,
    Alternate = 0b10,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum PullUpDown {
    None = 0b00,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum AlternateFunction {
    AF0 = 0b00,
}

#[derive(Debug)]
pub struct Gpio {
    pub port: Port,
    pub pin: u32,
}

impl Gpio {
    pub fn setup(&self, mode: Mode, pupd: PullUpDown) {
        let cs = unsafe { CriticalSection::new() };
        let cs = &cs;
        match self.port {
            Port::A => {
                stm32f0xx::GPIOA.borrow(cs).moder.modify(|r, w| unsafe {
                    w.bits(
                        r.bits() & !(0b11 << (self.pin * 2)) | ((mode as u32) << (self.pin * 2)),
                    )
                });
                stm32f0xx::GPIOA.borrow(cs).pupdr.modify(|r, w| unsafe {
                    w.bits(
                        r.bits() & !(0b11 << (self.pin * 2)) | ((pupd as u32) << (self.pin * 2)),
                    )
                });
            }
            Port::B => {
                stm32f0xx::GPIOB.borrow(cs).moder.modify(|r, w| unsafe {
                    w.bits(
                        r.bits() & !(0b11 << (self.pin * 2)) | ((mode as u32) << (self.pin * 2)),
                    )
                });
                stm32f0xx::GPIOB.borrow(cs).pupdr.modify(|r, w| unsafe {
                    w.bits(
                        r.bits() & !(0b11 << (self.pin * 2)) | ((pupd as u32) << (self.pin * 2)),
                    )
                });
            }
            Port::C => {
                stm32f0xx::GPIOC.borrow(cs).moder.modify(|r, w| unsafe {
                    w.bits(
                        r.bits() & !(0b11 << (self.pin * 2)) | ((mode as u32) << (self.pin * 2)),
                    )
                });
                stm32f0xx::GPIOC.borrow(cs).pupdr.modify(|r, w| unsafe {
                    w.bits(
                        r.bits() & !(0b11 << (self.pin * 2)) | ((pupd as u32) << (self.pin * 2)),
                    )
                });
            }
            Port::D => {
                stm32f0xx::GPIOD.borrow(cs).moder.modify(|r, w| unsafe {
                    w.bits(
                        r.bits() & !(0b11 << (self.pin * 2)) | ((mode as u32) << (self.pin * 2)),
                    )
                });
                stm32f0xx::GPIOD.borrow(cs).pupdr.modify(|r, w| unsafe {
                    w.bits(
                        r.bits() & !(0b11 << (self.pin * 2)) | ((pupd as u32) << (self.pin * 2)),
                    )
                });
            }
            Port::E => {
                stm32f0xx::GPIOE.borrow(cs).moder.modify(|r, w| unsafe {
                    w.bits(
                        r.bits() & !(0b11 << (self.pin * 2)) | ((mode as u32) << (self.pin * 2)),
                    )
                });
                stm32f0xx::GPIOE.borrow(cs).pupdr.modify(|r, w| unsafe {
                    w.bits(
                        r.bits() & !(0b11 << (self.pin * 2)) | ((pupd as u32) << (self.pin * 2)),
                    )
                });
            }
            Port::F => {
                stm32f0xx::GPIOF.borrow(cs).moder.modify(|r, w| unsafe {
                    w.bits(
                        r.bits() & !(0b11 << (self.pin * 2)) | ((mode as u32) << (self.pin * 2)),
                    )
                });
                stm32f0xx::GPIOF.borrow(cs).pupdr.modify(|r, w| unsafe {
                    w.bits(
                        r.bits() & !(0b11 << (self.pin * 2)) | ((pupd as u32) << (self.pin * 2)),
                    )
                });
            }
        }
    }

    pub fn set_alternate_function(&self, af: AlternateFunction) {
        let low = self.pin < 16;

        let cs = unsafe { CriticalSection::new() };
        let cs = &cs;

        match self.port {
            Port::A => if low {
                stm32f0xx::GPIOA.borrow(cs).afrl.modify(|r, w| unsafe {
                    w.bits(
                        r.bits() & !(0b1111 << (self.pin * 4)) | ((af as u32) << (self.pin * 4)),
                    )
                });
            } else {
                stm32f0xx::GPIOA.borrow(cs).afrh.modify(|r, w| unsafe {
                    w.bits(
                        r.bits() & !(0b1111 << (self.pin * 4)) | ((af as u32) << (self.pin * 4)),
                    )
                });
            },
            Port::B => if low {
                stm32f0xx::GPIOB.borrow(cs).afrl.modify(|r, w| unsafe {
                    w.bits(
                        r.bits() & !(0b1111 << (self.pin * 4)) | ((af as u32) << (self.pin * 4)),
                    )
                });
            } else {
                stm32f0xx::GPIOB.borrow(cs).afrh.modify(|r, w| unsafe {
                    w.bits(
                        r.bits() & !(0b1111 << (self.pin * 4)) | ((af as u32) << (self.pin * 4)),
                    )
                });
            },
            Port::C => if low {
                stm32f0xx::GPIOC.borrow(cs).afrl.modify(|r, w| unsafe {
                    w.bits(
                        r.bits() & !(0b1111 << (self.pin * 4)) | ((af as u32) << (self.pin * 4)),
                    )
                });
            } else {
                stm32f0xx::GPIOC.borrow(cs).afrh.modify(|r, w| unsafe {
                    w.bits(
                        r.bits() & !(0b1111 << (self.pin * 4)) | ((af as u32) << (self.pin * 4)),
                    )
                });
            },
            Port::D => if low {
                stm32f0xx::GPIOD.borrow(cs).afrl.modify(|r, w| unsafe {
                    w.bits(
                        r.bits() & !(0b1111 << (self.pin * 4)) | ((af as u32) << (self.pin * 4)),
                    )
                });
            } else {
                stm32f0xx::GPIOD.borrow(cs).afrh.modify(|r, w| unsafe {
                    w.bits(
                        r.bits() & !(0b1111 << (self.pin * 4)) | ((af as u32) << (self.pin * 4)),
                    )
                });
            },
            Port::E => if low {
                stm32f0xx::GPIOE.borrow(cs).afrl.modify(|r, w| unsafe {
                    w.bits(
                        r.bits() & !(0b1111 << (self.pin * 4)) | ((af as u32) << (self.pin * 4)),
                    )
                });
            } else {
                stm32f0xx::GPIOE.borrow(cs).afrh.modify(|r, w| unsafe {
                    w.bits(
                        r.bits() & !(0b1111 << (self.pin * 4)) | ((af as u32) << (self.pin * 4)),
                    )
                });
            },
            Port::F => if low {
                stm32f0xx::GPIOF.borrow(cs).afrl.modify(|r, w| unsafe {
                    w.bits(
                        r.bits() & !(0b1111 << (self.pin * 4)) | ((af as u32) << (self.pin * 4)),
                    )
                });
            } else {
                stm32f0xx::GPIOF.borrow(cs).afrh.modify(|r, w| unsafe {
                    w.bits(
                        r.bits() & !(0b1111 << (self.pin * 4)) | ((af as u32) << (self.pin * 4)),
                    )
                });
            },
        }
    }
}
