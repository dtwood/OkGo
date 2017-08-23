use stm32f0xx;
use bare_metal::CriticalSection;
use gpio::{Gpio, Mode, PullUpDown, Port};

pub fn set_up_adc() {
    let cs = unsafe { CriticalSection::new() };
    let cs = &cs;
    let adc = stm32f0xx::ADC.borrow(cs);
    adc.cr.write(|w| w.addis().set_bit());
    while adc.cr.read().aden().bit_is_set() { /* Wait for the ADC to be disabled */ }

    adc.cfgr2.reset();

    adc.cr.write(|w| w.adcal().set_bit());
    while adc.cr.read().adcal().bit_is_set() { /* Wait for calibration to finish */ }

    unsafe {
        adc.cfgr1.write(|w| {
            w.cont()
                .clear_bit()
                .discen()
                .set_bit()
                .align()
                .set_bit()
                .res()
                .bits(0)
        });

        adc.smpr.write(|w| w.smpr().bits(6));
    }

    adc.cr.write(|w| w.aden().set_bit());
    while adc.cr.read().aden().bit_is_clear() { /* Wait for the ADC to be enabled */ }
}

pub struct Adc {
    pub gpio: Gpio,
    pub channel: u8,
}

impl Adc {
    pub const fn new(port: Port, pin: u32, channel: u8) -> Adc {
        Adc {
            gpio: Gpio {
                port: port,
                pin: pin,
            },
            channel: channel,
        }
    }

    pub fn setup(&self) {
        assert!(self.channel <= 18);

        self.gpio.setup(Mode::Analog, PullUpDown::None);
    }

    pub fn read(&self) -> u16 {
        let cs = unsafe { CriticalSection::new() };
        let cs = &cs;
        let adc = stm32f0xx::ADC.borrow(cs);

        adc.chselr
            .write(|w| unsafe { w.bits(1 << self.channel) });

        adc.cr.write(|w| w.adstart().set_bit());
        while adc.isr.read().eoc().bit_is_clear() { /* Wait for the conversion to finish */ }

        adc.dr.read().data().bits()
    }
}
