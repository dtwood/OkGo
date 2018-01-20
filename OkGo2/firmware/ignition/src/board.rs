use f0::gpio::{Analog, Output, PushPull};
use f0::gpio::{gpioa, gpiob};
use f0::gpio;
use f0::spi::Spi;
use stm32f0xx::SPI1;

pub type Beeper = gpioa::PA4<Analog>;

pub type LedGreen = gpiob::PB13<Output<PushPull>>;
pub type LedYellow = gpiob::PB12<Output<PushPull>>;
pub type LedArm = gpiob::PB8<Output<PushPull>>;
pub type LedDisarm = gpiob::PB9<Output<PushPull>>;

pub type RelayUpstream = gpioa::PA10<Output<PushPull>>;
pub type RelayChannel1 = gpioa::PA9<Output<PushPull>>;
pub type RelayChannel2 = gpioa::PA8<Output<PushPull>>;
pub type RelayChannel3 = gpiob::PB15<Output<PushPull>>;
pub type RelayChannel4 = gpiob::PB14<Output<PushPull>>;

pub type RadioSpi = Spi<
    SPI1,
    (
        gpiob::PB3<gpio::AF0>, // SCK
        gpiob::PB4<gpio::AF0>, // MISO
        gpiob::PB5<gpio::AF0>, // MOSI
    ),
>;
pub type RadioNss = gpioa::PA15<Output<PushPull>>;

pub type BattMon = gpioa::PA0<Analog>;
pub type RelaySense = gpiob::PB1<Analog>;
pub type ContCh1 = gpiob::PB0<Analog>;
pub type ContCh2 = gpioa::PA7<Analog>;
pub type ContCh3 = gpioa::PA6<Analog>;
pub type ContCh4 = gpioa::PA5<Analog>;
