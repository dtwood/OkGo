use firmware_common::adc::adc_to_millivolts;
use rtfm::{self, Resource};

/// Convert raw ADC value to continuity ohms
pub fn adc_to_ohms(raw: u16) -> u8 {
    let mut millivolts = adc_to_millivolts(raw);

    if millivolts >= 3300 {
        millivolts = 3299; /* Avoid negatives or div by zero */
    }

    /* Remove 48mV offset caused by diode reverse current */
    if millivolts >= 48 {
        millivolts -= 48;
    } else {
        millivolts = 0;
    }

    let ohms = millivolts * 1000 / (3300 - millivolts);

    if ohms >= 10_000 {
        // magic value meaning open
        255
    } else if ohms >= 255 {
        // magic value meaning high but not open
        254
    } else {
        ohms as u8
    }
}

pub fn adc_to_battery_voltage(raw: u16) -> u8 {
    let mut adc_val: u32 = adc_to_millivolts(raw);

    adc_val = adc_val * 133 / 33;

    // Round instead of truncate
    if (adc_val / 10) % 10 >= 5 {
        (adc_val / 100 + 1) as u8
    } else {
        (adc_val / 100) as u8
    }
}

pub fn delay_ms(t: &rtfm::Threshold, millis: ::_resource::MILLIS, time: u32) {
    let start = **millis.borrow(t);
    while **millis.borrow(t) < start + time {
        rtfm::wfi();
    }
}
