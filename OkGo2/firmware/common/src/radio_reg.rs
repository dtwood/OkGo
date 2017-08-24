# ! [ feature ( abi_msp430_interrupt ) ] # ! [ cfg_attr ( feature = "rt" , feature ( global_asm ) ) ] # ! [ cfg_attr ( feature = "rt" , feature ( macro_reexport ) ) ] # ! [ cfg_attr ( feature = "rt" , feature ( used ) ) ] # ! [ doc = "Peripheral access API for RFM microcontrollers (generated using svd2rust v0.11.3)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.11.3/svd2rust/#peripheral-api" ] # ! [ deny ( missing_docs ) ] # ! [ deny ( warnings ) ] # ! [ allow ( non_camel_case_types ) ] # ! [ feature ( const_fn ) ] # ! [ no_std ]
extern crate bare_metal;
#[macro_reexport(default_handler)]
#[cfg(feature = "rt")]
extern crate msp430_rt;
extern crate vcell;
use core::ops::Deref;
use bare_metal::Peripheral;
#[doc = "32 Timer / Counter, counting up or down from different sources"]
pub const TIMER0: Peripheral<TIMER0> = unsafe { Peripheral::new(0) };
#[doc = "32 Timer / Counter, counting up or down from different sources"]
pub mod timer0 {
    use vcell::VolatileCell;
    #[doc = r" Register block"]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "0x00 - Control Register"] pub cr: CR,
    }
    #[doc = "Control Register"]
    pub struct CR {
        register: VolatileCell<u32>,
    }
    #[doc = "Control Register"]
    pub mod cr {
        #[doc = r" Value read from the register"]
        pub struct R {
            bits: u32,
        }
        #[doc = r" Value to write to the register"]
        pub struct W {
            bits: u32,
        }
        impl super::CR {
            #[doc = r" Modifies the contents of the register"]
            #[inline(always)]
            pub fn modify<F>(&self, f: F)
            where
                for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
            {
                let bits = self.register.get();
                let r = R { bits: bits };
                let mut w = W { bits: bits };
                f(&r, &mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Reads the contents of the register"]
            #[inline(always)]
            pub fn read(&self) -> R {
                R {
                    bits: self.register.get(),
                }
            }
            #[doc = r" Writes to the register"]
            #[inline(always)]
            pub fn write<F>(&self, f: F)
            where
                F: FnOnce(&mut W) -> &mut W,
            {
                let mut w = W::reset_value();
                f(&mut w);
                self.register.set(w.bits);
            }
            #[doc = r" Writes the reset value to the register"]
            #[inline(always)]
            pub fn reset(&self) {
                self.write(|w| w)
            }
        }
        #[doc = "Possible values of the field `EN`"]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum ENR {
            #[doc = "Timer is disabled and does not operate"] DISABLE,
            #[doc = "Timer is enabled and can operate"] ENABLE,
        }
        impl ENR {
            #[doc = r" Returns `true` if the bit is clear (0)"]
            #[inline(always)]
            pub fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            #[doc = r" Returns `true` if the bit is set (1)"]
            #[inline(always)]
            pub fn bit_is_set(&self) -> bool {
                self.bit()
            }
            #[doc = r" Value of the field as raw bits"]
            #[inline(always)]
            pub fn bit(&self) -> bool {
                match *self {
                    ENR::DISABLE => false,
                    ENR::ENABLE => true,
                }
            }
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _from(value: bool) -> ENR {
                match value {
                    false => ENR::DISABLE,
                    true => ENR::ENABLE,
                }
            }
            #[doc = "Checks if the value of the field is `DISABLE`"]
            #[inline(always)]
            pub fn is_disable(&self) -> bool {
                *self == ENR::DISABLE
            }
            #[doc = "Checks if the value of the field is `ENABLE`"]
            #[inline(always)]
            pub fn is_enable(&self) -> bool {
                *self == ENR::ENABLE
            }
        }
        #[doc = "Values that can be written to the field `EN`"]
        pub enum ENW {
            #[doc = "Timer is disabled and does not operate"] DISABLE,
            #[doc = "Timer is enabled and can operate"] ENABLE,
        }
        impl ENW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match *self {
                    ENW::DISABLE => false,
                    ENW::ENABLE => true,
                }
            }
        }
        #[doc = r" Proxy"]
        pub struct _ENW<'a> {
            w: &'a mut W,
        }
        impl<'a> _ENW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: ENW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = "Timer is disabled and does not operate"]
            #[inline(always)]
            pub fn disable(self) -> &'a mut W {
                self.variant(ENW::DISABLE)
            }
            #[doc = "Timer is enabled and can operate"]
            #[inline(always)]
            pub fn enable(self) -> &'a mut W {
                self.variant(ENW::ENABLE)
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 0;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }
        #[doc = "Values that can be written to the field `RST`"]
        pub enum RSTW {
            #[doc = "Reset the Timer"] RESET_TIMER,
        }
        impl RSTW {
            #[allow(missing_docs)]
            #[doc(hidden)]
            #[inline(always)]
            pub fn _bits(&self) -> bool {
                match *self {
                    RSTW::RESET_TIMER => true,
                }
            }
        }
        #[doc = r" Proxy"]
        pub struct _RSTW<'a> {
            w: &'a mut W,
        }
        impl<'a> _RSTW<'a> {
            #[doc = r" Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: RSTW) -> &'a mut W {
                {
                    self.bit(variant._bits())
                }
            }
            #[doc = "Reset the Timer"]
            #[inline(always)]
            pub fn reset_timer(self) -> &'a mut W {
                self.variant(RSTW::RESET_TIMER)
            }
            #[doc = r" Sets the field bit"]
            pub fn set_bit(self) -> &'a mut W {
                self.bit(true)
            }
            #[doc = r" Clears the field bit"]
            pub fn clear_bit(self) -> &'a mut W {
                self.bit(false)
            }
            #[doc = r" Writes raw bits to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut W {
                const MASK: bool = true;
                const OFFSET: u8 = 1;
                self.w.bits &= !((MASK as u32) << OFFSET);
                self.w.bits |= ((value & MASK) as u32) << OFFSET;
                self.w
            }
        }
        impl R {
            #[doc = r" Value of the register as raw bits"]
            #[inline(always)]
            pub fn bits(&self) -> u32 {
                self.bits
            }
            #[doc = "Bit 0 - Enable"]
            #[inline(always)]
            pub fn en(&self) -> ENR {
                ENR::_from({
                    const MASK: bool = true;
                    const OFFSET: u8 = 0;
                    ((self.bits >> OFFSET) & MASK as u32) != 0
                })
            }
        }
        impl W {
            #[doc = r" Reset value of the register"]
            #[inline(always)]
            pub fn reset_value() -> W {
                W { bits: 0 }
            }
            #[doc = r" Writes raw bits to the register"]
            #[inline(always)]
            pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
                self.bits = bits;
                self
            }
            #[doc = "Bit 0 - Enable"]
            #[inline(always)]
            pub fn en(&mut self) -> _ENW {
                _ENW { w: self }
            }
            #[doc = "Bit 1 - Reset Timer"]
            #[inline(always)]
            pub fn rst(&mut self) -> _RSTW {
                _RSTW { w: self }
            }
        }
    }
}
#[doc = "32 Timer / Counter, counting up or down from different sources"]
pub struct TIMER0 {
    register_block: timer0::RegisterBlock,
}
impl Deref for TIMER0 {
    type Target = timer0::RegisterBlock;
    fn deref(&self) -> &timer0::RegisterBlock {
        &self.register_block
    }
}
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals<'a> {
    #[doc = "TIMER0"] pub TIMER0: &'a TIMER0,
}
impl<'a> Peripherals<'a> {
    #[doc = r" Grants access to all the peripherals"]
    pub unsafe fn all() -> Self {
        Peripherals {
            TIMER0: &*TIMER0.get(),
        }
    }
}
