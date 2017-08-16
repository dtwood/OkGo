use ignition;
use io::ignition_buzzer_set;
use bare_metal::CriticalSection;

/// Do beeping
#[no_mangle]
pub unsafe extern "C" fn do_beep(state: *mut ignition::State) {
    let cs = CriticalSection::new();
    do_beep_rust(&cs, &mut *state);
}

fn do_beep_rust(cs: &CriticalSection, state: &mut ignition::State) {
    let beep_period: u32;
    let beep_len: u32;

    if state.fire_ch1 || state.fire_ch2 || state.fire_ch3 || state.fire_ch4 {
        beep_period = 200;
        beep_len = 50
    } else if state.armed {
        beep_period = 500;
        beep_len = 250;
    } else {
        beep_period = 1000;
        beep_len = 50;
    }

    let time = unsafe { get_millis() };
    if time - state.beep_start > beep_period {
        // Start a new beep with the high cycle
        state.beep_start = time;

        match state.beep_volume {
            0 => ignition_buzzer_set(cs, 0), // No beep
            1 => ignition_buzzer_set(cs, 94), // Low beep
            2 => ignition_buzzer_set(cs, 255), // Deafening beep
            _ => ignition_buzzer_set(cs, 112), // Medium beep
        }
    } else if time - state.beep_start > beep_len {
        // Do the low cycle of the beep
        ignition_buzzer_set(cs, 0); // Off
    }
}

extern "C" {
    fn get_millis() -> u32;
}
