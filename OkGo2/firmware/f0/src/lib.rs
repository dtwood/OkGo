#![no_std]

extern crate bare_metal;
extern crate stm32f0xx;

#[macro_use]
pub mod adc;
#[macro_use]
pub mod dac;
pub mod gpio;
#[macro_use]
pub mod out;
pub mod spi;
