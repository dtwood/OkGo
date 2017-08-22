use stm32f0xx;
use bare_metal::CriticalSection;
use gpio::{Gpio, Mode, PullUpDown};

#[macro_export]
macro_rules! adc_pin {
    ($id: ident, $adc: expr, $port: ident, $pin: expr, $channel: expr) => {
        pub static $id: $crate::adc::AdcPin = $crate::adc::AdcPin {
            gpio: $crate::gpio::Gpio {
                port: $crate::gpio::Port::$port,
                pin: $pin,
            },
            channel: $channel,
            adc: &$adc,
        };
    }
}

#[macro_export]
macro_rules! adc {
    ($id: ident) => {
        pub static $id: $crate::adc::Adc = $crate::adc::Adc {
        };
    }
}

pub struct Adc {
}

pub struct AdcPin {
    pub gpio: Gpio,
    pub channel: u8,
    pub adc: &'static Adc,
}

impl Adc {
    pub fn setup(&self, cs: &CriticalSection) {
        let adc = stm32f0xx::ADC.borrow(&cs);
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

    fn read(&self, cs: &CriticalSection, adc_pin: &AdcPin) -> u16 {
        let adc = stm32f0xx::ADC.borrow(&cs);

        adc.chselr.write(|w| unsafe { w.bits(1 << adc_pin.channel) });

        adc.cr.write(|w| w.adstart().set_bit());
        while adc.isr.read().eoc().bit_is_clear() { /* Wait for the conversion to finish */ }

        adc.dr.read().data().bits()
    }
}

impl AdcPin {
    pub fn setup(&self, cs: &CriticalSection) {
        assert!(self.channel <= 18);

        self.gpio.setup(cs, Mode::Analog, PullUpDown::None);
    }

    pub fn read(&self, cs: &CriticalSection) -> u16 {
        self.adc.read(cs, self)
    }
}
