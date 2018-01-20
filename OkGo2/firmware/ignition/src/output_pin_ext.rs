use embedded_hal::digital::OutputPin;

pub trait OutputPinExt {
    fn toggle(&mut self);
}

impl<T: OutputPin> OutputPinExt for T {
    fn toggle(&mut self) {
        if self.is_low() {
            self.set_high();
        } else {
            self.set_low();
        }
    }
}
