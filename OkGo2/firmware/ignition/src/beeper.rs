// use f0::dac::Dac;
use rtfm;
use f0::gpio::Port;
use stm32f0xx;

#[derive(Copy, Clone, Debug)]
pub struct Beeper {
    start: u32,
    period: u32,
    len: u32,
    /// Good values for this are:
    /// 0 - off
    /// 93 - low
    /// 112 - medium
    /// 255 - deafening
    /// The middle two might be variable or sensitive to temperature
    volume: u8,
}

#[derive(Copy, Clone, Debug)]
pub enum Rate {
    Fast,
    Slow,
}

#[derive(Copy, Clone, Debug)]
pub enum Volume {
    Silent,
    Low,
    Medium,
    Loud,
}

impl Beeper {
    pub const fn new() -> Beeper {
        Beeper {
            start: 0,
            period: 1000,
            len: 50,
            volume: 255,
        }
    }

    pub fn init(&self, dac: &stm32f0xx::DAC, port: Port, pin: u32) {
        dac.init(port, pin);
        dac.cr.write(|w| w.ten1().clear_bit().en1().set_bit());
    }

    pub fn set_rate(&mut self, rate: Rate) {
        match rate {
            Rate::Fast => {
                self.period = 200;
                self.len = 50;
            }
            Rate::Slow => {
                self.period = 1000;
                self.len = 50;
            }
        }
    }

    pub fn set_volume(&mut self, volume: Volume) {
        self.volume = match volume {
            Volume::Silent => 0,   // No beep
            Volume::Low => 94,     // Low beep
            Volume::Medium => 112, // Medium beep
            Volume::Loud => 255,   // Deafening beep
        }
    }

    pub fn do_beep<M, DAC>(&mut self, t: &mut rtfm::Threshold, millis: &M, dac: &DAC)
    where
        M: rtfm::Resource<Data = u32>,
        DAC: rtfm::Resource<Data = stm32f0xx::DAC>,
    {
        let time = **millis.borrow(t);

        if time - self.start > self.period {
            // Start a new beep with the high cycle
            self.start = time;

            dac.claim(t, |dac, _| dac.set_right_u8(self.volume));
        } else if time - self.start > self.len {
            // Do the low cycle of the beep
            dac.claim(t, |dac, _| dac.set_right_u8(0));
        }
    }
}
