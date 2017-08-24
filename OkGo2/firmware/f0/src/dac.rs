use stm32f0xx;
use gpio::{Gpio, Mode, Port, PullUpDown};

pub trait Dac {
    fn init(&self, port: Port, pin: u32);
    fn set_right_u8(&self, value: u8);
}

impl Dac for stm32f0xx::DAC {
    fn init(&self, port: Port, pin: u32) {
        Gpio {
            port: port,
            pin: pin,
        }.setup(Mode::Analog, PullUpDown::None);
    }

    fn set_right_u8(&self, value: u8) {
        self.dhr8r1.write(|w| unsafe { w.dacc1dhr().bits(value) });
    }
}
