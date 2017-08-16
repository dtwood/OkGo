use stm32f0xx;
use bare_metal::CriticalSection;
use gpio::{Gpio, Mode, PullUpDown};

#[macro_export]
macro_rules! adc {
    ($id: ident, $port: ident, $pin: expr, $channel: expr) => {
        pub static $id: Adc = Adc {
            gpio: Gpio {
                port: Port::$port,
                pin: $pin,
            },
            channel: $channel,
        };
    }
}

/// General purpose ADC initialisation and calibration
#[no_mangle]
pub unsafe extern "C" fn adc_init() {
    let cs = CriticalSection::new();
    init(&cs);
}

pub fn init(cs: &CriticalSection) {
        let adc = stm32f0xx::ADC.borrow(&cs);

        adc.cr.write(|w| w.addis().set_bit());
        while adc.cr.read().aden().bit_is_set() {
            /* Wait for the ADC to be disabled */
        }

        adc.cfgr2.reset();

        adc.cr.write(|w| w.adcal().set_bit());
        while adc.cr.read().adcal().bit_is_set() {
            /* Wait for calibration to finish */
        }

            unsafe {
        adc.cfgr1.write(|w| w
            .cont().clear_bit()
            .discen().set_bit()
            .align().set_bit()
            .res().bits(0)
        );

        adc.smpr.write(|w| w
            .smpr().bits(6)
        );
    }

        adc.cr.write(|w| w.aden().set_bit());
        while adc.cr.read().aden().bit_is_clear() {
            /* Wait for the ADC to be enabled */
        }
}

/// Read an ADC value, blocking and returning result
#[no_mangle]
pub unsafe extern "C" fn adc_read(channel: u8) -> u16 {
    let cs = CriticalSection::new();
    let adc = stm32f0xx::ADC.borrow(&cs);
    read(adc, channel)
}

/// Read an ADC value, blocking and returning result
fn read(adc: &stm32f0xx::ADC, channel: u8) -> u16 {
    assert!(channel <= 18);

    adc.chselr.write(|w| unsafe { w
        .bits(1 << channel)
    });

    adc.cr.write(|w| w
        .adstart().set_bit()
    );
    while adc.isr.read().eoc().bit_is_clear() {
        /* Wait for the conversion to finish */
    }

    adc.dr.read().data().bits()
}

pub struct Adc {
    pub gpio: Gpio,
    pub channel: u8,
}

impl Adc {
    pub fn read(&self, cs: &CriticalSection) -> u16 {
        read(stm32f0xx::ADC.borrow(&cs), self.channel)
    }

    pub fn setup(&self, cs: &CriticalSection) {
        self.gpio.setup(cs, Mode::Analog, PullUpDown::None)
    }
}
