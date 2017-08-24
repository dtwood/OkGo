#![feature(const_fn)]
#![no_std]

extern crate bare_metal;
extern crate cortex_m_rtfm as rtfm;
extern crate embedded_hal;
extern crate nb;
extern crate stm32f0xx;

pub mod adc;
pub mod dac;
pub mod gpio;
pub mod output;
pub mod spi;
pub mod input;
