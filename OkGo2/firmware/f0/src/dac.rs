use stm32f0xx;
use gpio::{Gpio, Mode, PullUpDown, Port};
use bare_metal::CriticalSection;

pub struct Dac {
    pub gpio: Gpio,
}

impl Dac {
    pub const fn new(port: Port, pin: u32) -> Dac {
        Dac {
            gpio: Gpio {
                port: port,
                pin: pin,
            },
        }
    }

    pub fn set_right_u8(&self, value: u8) {
        let cs = unsafe { CriticalSection::new() };
        let cs = &cs;
        stm32f0xx::DAC
            .borrow(cs)
            .dhr8r1
            .write(|w| unsafe { w.dacc1dhr().bits(value) });
    }

    pub fn setup(&self) {
        self.gpio.setup(Mode::Analog, PullUpDown::None);
    }
}
