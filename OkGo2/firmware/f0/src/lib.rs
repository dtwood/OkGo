#![feature(const_fn)]
#![no_std]

extern crate bare_metal;
extern crate embedded_hal;
extern crate nb;
extern crate stm32f0xx;

#[macro_use]
pub mod adc;
#[macro_use]
pub mod dac;
pub mod gpio;
#[macro_use]
pub mod output;
pub mod spi;
#[macro_use]
pub mod input;
