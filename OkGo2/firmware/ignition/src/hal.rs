use embedded_hal::digital::OutputPin;
use f0;
use f0::gpio::Output;
use f0::gpio::PushPull;

pub trait Led {
    fn on(&mut self);
    fn off(&mut self);
    fn toggle(&mut self);
    fn set_bool(&mut self, value: bool);
}

pub type LedGreen = f0::gpio::gpiob::PB13<Output<PushPull>>;
pub type LedYellow = f0::gpio::gpiob::PB12<Output<PushPull>>;
pub type LedArm = f0::gpio::gpiob::PB8<Output<PushPull>>;
pub type LedDisarm = f0::gpio::gpiob::PB9<Output<PushPull>>;

pub trait LedPin {}

impl LedPin for LedGreen {}
impl LedPin for LedYellow {}
impl LedPin for LedArm {}
impl LedPin for LedDisarm {}

impl<T> Led for T
where
    T: OutputPin + LedPin,
{
    fn on(&mut self) {
        self.set_high();
    }
    fn off(&mut self) {
        self.set_low();
    }
    fn toggle(&mut self) {
        if self.is_high() {
            self.set_low();
        } else {
            self.set_high();
        }
    }
    fn set_bool(&mut self, value: bool) {
        if value {
            self.set_high();
        } else {
            self.set_low();
        }
    }
}

pub trait Relay {
    fn close(&mut self);
    fn open(&mut self);
}

pub type RelayUpstream = f0::gpio::gpioa::PA10<Output<PushPull>>;
pub type RelayChannel1 = f0::gpio::gpioa::PA9<Output<PushPull>>;
pub type RelayChannel2 = f0::gpio::gpioa::PA8<Output<PushPull>>;
pub type RelayChannel3 = f0::gpio::gpiob::PB15<Output<PushPull>>;
pub type RelayChannel4 = f0::gpio::gpiob::PB14<Output<PushPull>>;

pub trait RelayPin {}

impl RelayPin for RelayUpstream {}
impl RelayPin for RelayChannel1 {}
impl RelayPin for RelayChannel2 {}
impl RelayPin for RelayChannel3 {}
impl RelayPin for RelayChannel4 {}

impl<T> Relay for T
where
    T: OutputPin + RelayPin,
{
    fn close(&mut self) {
        self.set_high();
    }
    fn open(&mut self) {
        self.set_low();
    }
}

// pub struct RelayA(u32);
// pub struct RelayB(u32);
//
// pub trait Relay {
//     type GPIO: OutputT + GpioT;
//
//     fn pin(&self) -> u32;
//
//     fn init(&self, gpio: &Self::GPIO, rcc: &stm32f0xx::RCC) {
//         gpio.init(rcc);
//         gpio.set_output(self.pin());
//     }
//     fn close(&self, gpio: &Self::GPIO) {
//         gpio.high(self.pin())
//     }
//     fn open(&self, gpio: &Self::GPIO) {
//         gpio.low(self.pin())
//     }
//     fn set_bool(&self, gpio: &Self::GPIO, value: bool) {
//         gpio.set_bool(self.pin(), value);
//     }
// }
//
// impl Relay for RelayA {
//     type GPIO = stm32f0xx::GPIOA;
//
//     fn pin(&self) -> u32 {
//         self.0
//     }
// }
//
// impl Relay for RelayB {
//     type GPIO = stm32f0xx::GPIOB;
//
//     fn pin(&self) -> u32 {
//         self.0
//     }
// }
//
// pub mod relays {
//     use super::*;
//
// }
//
// pub mod leds {
//     use super::*;
//
// }
