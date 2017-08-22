#![no_std]
#![feature(lang_items, unwind_attributes)]
#![feature(compiler_builtins_lib)]
#![feature(never_type)]
#![no_main]

extern crate libopencm3_sys;
extern crate firmware_common;
extern crate rlibc;
extern crate compiler_builtins;
#[macro_use]
extern crate f0;
extern crate bare_metal;
extern crate stm32f0xx;
extern crate hmac;
extern crate md_5;
extern crate nb;

pub mod beep;
pub mod radio;
pub mod ignition;
pub mod io;

use bare_metal::CriticalSection;
use firmware_common::utils::{delay_ms, get_millis};

/// Drop delay in ms
const PACKET_DROP_DELAY: u32 = 5000;

#[no_mangle]
pub unsafe extern "C" fn main() -> i32 {
    rust_main();
}

fn rust_main() -> ! {
    let cs = unsafe { CriticalSection::new() };
    let mut beeper = beep::Beeper::new();
    let mut last_packet: u32 = 0;

    ignition::init(&cs);

    io::LED_GREEN.clear(&cs);
    io::LED_YELLOW.clear(&cs);

    radio::make_ready();

    loop {
        match radio::receive_async() {
            Ok(state) => {
                beeper.set_state(&state);

                if state.armed {
                    io::LED_ARM.set(&cs);
                    io::LED_DISARM.clear(&cs);
                    io::UPSTREAM_RELAY.set(&cs);
                    io::FIRE_CH1.set_bool(&cs, state.fire_ch1);
                    io::FIRE_CH2.set_bool(&cs, state.fire_ch2);
                    io::FIRE_CH3.set_bool(&cs, state.fire_ch3);
                    io::FIRE_CH4.set_bool(&cs, state.fire_ch4);
                } else {
                    io::LED_ARM.clear(&cs);
                    io::LED_DISARM.set(&cs);
                    io::UPSTREAM_RELAY.clear(&cs);
                    io::FIRE_CH1.clear(&cs);
                    io::FIRE_CH2.clear(&cs);
                    io::FIRE_CH3.clear(&cs);
                    io::FIRE_CH4.clear(&cs);
                }

                last_packet = get_millis();
                delay_ms(10);
                radio::transmit(&cs, &state);
                radio::make_ready();

                io::LED_GREEN.toggle(&cs);
                io::LED_YELLOW.toggle(&cs);
            }
            Err(nb::Error::WouldBlock) => if get_millis() - last_packet > PACKET_DROP_DELAY {
                io::LED_ARM.clear(&cs);
                io::LED_DISARM.set(&cs);
                io::UPSTREAM_RELAY.clear(&cs);
                io::FIRE_CH1.clear(&cs);
                io::FIRE_CH2.clear(&cs);
                io::FIRE_CH3.clear(&cs);
                io::FIRE_CH4.clear(&cs);
            },
            Err(nb::Error::Other(_)) => {
                io::LED_ARM.clear(&cs);
                io::LED_DISARM.set(&cs);
                io::UPSTREAM_RELAY.clear(&cs);
                io::FIRE_CH1.clear(&cs);
                io::FIRE_CH2.clear(&cs);
                io::FIRE_CH3.clear(&cs);
                io::FIRE_CH4.clear(&cs);
            }
        }

        beeper.do_beep(&cs);
    }
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn rust_eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn rust_begin_unwind(_: core::fmt::Arguments, _: &'static str, _: u32) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn abort() -> ! {
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    rlibc::memcpy(dest, src, n)
}
