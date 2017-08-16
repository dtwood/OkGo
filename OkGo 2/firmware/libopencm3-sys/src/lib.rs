#![no_std]

extern "C" {
    pub fn gpio_set(gpioport: u32, gpios: u16);
    pub fn gpio_clear(gpioport: u32, gpios: u16);
    pub fn gpio_toggle(gpioport: u32, gpios: u16);
}
