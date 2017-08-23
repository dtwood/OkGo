use gpio::{AlternateFunction, Gpio, Mode, PullUpDown};
use embedded_hal;
use nb;

#[derive(Debug)]
pub struct Spi {
    pub sck: Gpio,
    pub miso: Gpio,
    pub mosi: Gpio,
}

/// SPI error
#[derive(Debug)]
pub enum Error {
    /// Overrun occurred
    Overrun,
    #[doc(hidden)] _Extensible,
}

impl Spi {
    pub fn setup(&self) {
        self.sck.setup(Mode::Alternate, PullUpDown::None);
        self.miso.setup(Mode::Alternate, PullUpDown::None);
        self.mosi.setup(Mode::Alternate, PullUpDown::None);
        self.sck.set_alternate_function(AlternateFunction::AF0);
        self.miso.set_alternate_function(AlternateFunction::AF0);
        self.mosi.set_alternate_function(AlternateFunction::AF0);
    }
}

impl embedded_hal::Spi<u8> for Spi {
    type Error = Error;

    fn read(&self) -> Result<u8, nb::Error<Self::Error>> {
        unimplemented!();
    }

    fn send(&self, _byte: u8) -> Result<(), nb::Error<Self::Error>> {
        unimplemented!();
    }
}
