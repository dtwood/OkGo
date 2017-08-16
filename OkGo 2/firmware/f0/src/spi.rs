use gpio::{Gpio, Mode, PullUpDown, AlternateFunction};
use bare_metal::CriticalSection;

#[derive(Debug)]
pub struct Spi {
    pub sck: Gpio,
    pub miso: Gpio,
    pub mosi: Gpio,
}

impl Spi {
    pub fn setup(&self, cs: &CriticalSection) {
        self.sck.setup(cs, Mode::Alternate, PullUpDown::None);
        self.miso.setup(cs, Mode::Alternate, PullUpDown::None);
        self.mosi.setup(cs, Mode::Alternate, PullUpDown::None);
        self.sck.set_alternate_function(cs, AlternateFunction::AF0);
        self.miso.set_alternate_function(cs, AlternateFunction::AF0);
        self.mosi.set_alternate_function(cs, AlternateFunction::AF0);
    }
}
