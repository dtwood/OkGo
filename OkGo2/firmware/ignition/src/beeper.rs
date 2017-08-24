use f0::dac::Dac;
use rtfm;

pub struct Beeper {
    device: Dac,
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

pub enum Rate {
    Fast,
    Slow,
}

pub enum Volume {
    Silent,
    Low,
    Medium,
    Loud,
}

impl Beeper {
    pub const fn new(device: Dac) -> Beeper {
        Beeper {
            device: device,
            start: 0,
            period: 1000,
            len: 50,
            volume: 255,
        }
    }

    pub fn setup(&self) {
        self.device.setup();
    }

    pub fn set_state(&mut self, rate: Rate, volume: Volume) {
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

        self.volume = match volume {
            Volume::Silent => 0,   // No beep
            Volume::Low => 94,     // Low beep
            Volume::Medium => 112, // Medium beep
            Volume::Loud => 255,   // Deafening beep
        }
    }

    pub fn do_beep<M>(&mut self, t: &rtfm::Threshold, millis: &M)
    where
        M: rtfm::Resource<Data = u32>,
    {
        let time = **millis.borrow(t);

        if time - self.start > self.period {
            // Start a new beep with the high cycle
            self.start = time;

            self.device.set_right_u8(self.volume);
        } else if time - self.start > self.len {
            // Do the low cycle of the beep
            self.device.set_right_u8(0);
        }
    }
}
