use stm32f0xx;
use gpio::Port;
use bare_metal::CriticalSection;

/// Read an ADC value, blocking and returning result

pub struct Dac {
    pub port: Port,
    pub pin: u32,
}

impl Dac {
    pub fn set_right_u8(&self, cs: &CriticalSection, value: u8) {
        stm32f0xx::DAC.borrow(cs).dhr8r1.write(|w| unsafe { w.dacc1dhr().bits(value) });
    }

    pub fn setup(&self, cs: &CriticalSection) {
        match self.port {
            Port::A => {
                stm32f0xx::GPIOA.borrow(cs).moder.modify(|r, w| unsafe { w.bits(r.bits() | (0b11 << (self.pin * 2)))});
                stm32f0xx::GPIOA.borrow(cs).pupdr.modify(|r, w| unsafe { w.bits(r.bits() & !(0b11 << (self.pin * 2)))});
            }
            _ => panic!(),
        }
    }
}
