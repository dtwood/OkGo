use bare_metal::CriticalSection;
use stm32f0xx;

pub trait GpioT {
    fn init(&self, rcc: &stm32f0xx::RCC);
    fn set_mode(&self, pin: u32, mode: Mode);
    fn set_pupd(&self, pin: u32, pupd: PullUpDown);

    fn get_bool(&self, pin: u32) -> bool;

    fn set_output(&self, pin: u32) {
        self.set_mode(pin, Mode::Output);
    }
    fn set_input(&self, pin: u32) {
        self.set_mode(pin, Mode::Input);
    }
}

macro_rules! gpio {
    ($type: ty) => {
        impl GpioT for $type {
            fn init(&self, rcc: &stm32f0xx::RCC) {
                rcc.ahbenr.write(|w| {
                    w
                    .iopben().set_bit()
                });
            }

            fn set_mode(&self, pin: u32, mode: Mode) {
                self.moder.modify(|r, w| unsafe {
                    w.bits(
                        r.bits() & !(0b11 << (pin * 2)) | ((mode as u32) << (pin * 2)),
                    )
                });
            }

            fn set_pupd(&self, pin: u32, pupd: PullUpDown) {
                self.pupdr.modify(|r, w| unsafe {
                    w.bits(
                        r.bits() & !(0b11 << (pin * 2)) | ((pupd as u32) << (pin * 2)),
                    )
                });
            }

            fn get_bool(&self, pin: u32) -> bool {
                self.odr.read().bits() & (1 << pin) != 0
            }
        }
    }
}

gpio!(stm32f0xx::GPIOA);
gpio!(stm32f0xx::GPIOB);
gpio!(stm32f0xx::GPIOC);
gpio!(stm32f0xx::GPIOD);
gpio!(stm32f0xx::GPIOE);
gpio!(stm32f0xx::GPIOF);

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
