use f0::analog_hal::Dac;

#[derive(Copy, Clone, Debug)]
pub struct Beeper<DAC, PIN> {
    dac: DAC,
    pin: PIN,
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

impl<DAC, PIN> Beeper<DAC, PIN>
where
    for<'a> (&'a mut DAC, &'a mut PIN): Dac<u8>,
{
    pub fn new(dac: DAC, pin: PIN) -> Beeper<DAC, PIN> {
        Beeper {
            dac,
            pin,
            start: 0,
            period: 1000,
            len: 50,
            volume: 255,
        }
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

    pub fn do_beep(&mut self, millis: u32) {
        if millis - self.start > self.period {
            // Start a new beep with the high cycle
            self.start = millis;

            (&mut self.dac, &mut self.pin).set(self.volume);
        } else if millis - self.start > self.len {
            // Do the low cycle of the beep
            (&mut self.dac, &mut self.pin).set(0);
        }
    }
}
