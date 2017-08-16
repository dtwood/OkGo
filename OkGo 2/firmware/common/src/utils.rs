extern "C" {
    /// Delay for approximately a millisecond. Very roughly calibrated by eye to
    /// within about 20% precision.
    /// @param delay Number of milliseconds to delay.
    pub fn delay_ms(delay: u32);

    /// Delay for approximately a microsecond. Very roughly calibrated by eye to
    /// within about 20% precision.
    /// @param delay Number of microseconds to delay.
    pub fn delay_us(delay: u32);

    /// Set a GPIO to a boolean value
    pub fn gpio_set_bool(port: u32, pin: u16, value: bool);

    /// Get a boolean value from a GPIO
    pub fn gpio_get_bool(port: u32, pin: u16) -> bool;

    /// Setup systick
    pub fn systick_init();

    /// Get current millisecond timer value */
    pub fn get_millis() -> u32;
}
