use ignition;
use io::ignition_buzzer_set;
use bare_metal::CriticalSection;
use firmware_common::utils::get_millis;

pub struct Beeper {
    beep_start: u32,
    beep_period: u32,
    beep_len: u32,
    beep_volume: u8,
}

impl Beeper {
    pub fn new() -> Beeper {
        Beeper {
            beep_start: 0,
            beep_period: 1000,
            beep_len: 50,
            beep_volume: 255,
        }
    }

    pub fn set_state(&mut self, state: &ignition::State) {
        if state.fire_ch1 || state.fire_ch2 || state.fire_ch3 || state.fire_ch4 {
            self.beep_period = 200;
            self.beep_len = 50
        } else if state.armed {
            self.beep_period = 500;
            self.beep_len = 250;
        } else {
            self.beep_period = 1000;
            self.beep_len = 50;
        }
        self.beep_volume = match state.beep_volume {
            0 => 0, // No beep
            1 => 94, // Low beep
            2 => 255, // Deafening beep
            _ => 112, // Medium beep
        }
    }

    pub fn do_beep(&mut self, cs: &CriticalSection) {
        let time = get_millis();

        if time - self.beep_start > self.beep_period {
            // Start a new beep with the high cycle
            self.beep_start = time;

            ignition_buzzer_set(cs, self.beep_volume);
        } else if time - self.beep_start > self.beep_len {
            // Do the low cycle of the beep
            ignition_buzzer_set(cs, 0); // Off
        }
    }
}
