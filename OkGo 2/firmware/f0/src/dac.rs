use stm32f0xx;
use gpio::{Gpio, Mode, PullUpDown};
use bare_metal::CriticalSection;

#[macro_export]
macro_rules! dac {
    ($id: ident, $port: ident, $pin: expr) => {
        pub static $id: Dac = Dac {
            gpio: Gpio {
                port: Port::$port,
                pin: $pin,
            },
        };
    }
}

pub struct Dac {
    pub gpio: Gpio,
}

impl Dac {
    pub fn set_right_u8(&self, cs: &CriticalSection, value: u8) {
        stm32f0xx::DAC.borrow(cs).dhr8r1.write(|w| unsafe { w.dacc1dhr().bits(value) });
    }

    pub fn setup(&self, cs: &CriticalSection) {
        self.gpio.setup(cs, Mode::Analog, PullUpDown::None);
    }
}
