mod internal {
    extern "C" {
        pub fn delay_ms(delay: u32);
        pub fn delay_us(delay: u32);
        pub fn systick_init();
        pub fn get_millis() -> u32;
    }
}

/// Delay for approximately a millisecond. Very roughly calibrated by eye to
/// within about 20% precision.
/// @param delay Number of milliseconds to delay.
pub fn delay_ms(delay: u32) {
    unsafe {
        internal::delay_ms(delay);
    }
}

/// Delay for approximately a microsecond. Very roughly calibrated by eye to
/// within about 20% precision.
/// @param delay Number of microseconds to delay.
pub fn delay_us(delay: u32) {
    unsafe {
        internal::delay_us(delay);
    }
}

/// Setup systick
pub fn systick_init() {
    unsafe {
        internal::systick_init();
    }
}

/// Get current millisecond timer value
pub fn get_millis() -> u32 {
    unsafe {
        internal::get_millis()
    }
}
