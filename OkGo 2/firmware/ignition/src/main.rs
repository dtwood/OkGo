#![no_std]
#![feature(lang_items, unwind_attributes)]
#![feature(compiler_builtins_lib)]
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

pub mod beep;
pub mod radio;
pub mod ignition;
pub mod io;

use core::mem;

use bare_metal::CriticalSection;
use firmware_common::rfm;
use firmware_common::utils::{get_millis, delay_ms};

/// Drop delay in ms
const PACKET_DROP_DELAY: u32 = 5000;

#[no_mangle]
pub unsafe extern "C" fn main() -> i32 {
    let cs = CriticalSection::new();
    let mut state: ignition::State = mem::uninitialized();
    let mut radio_state: radio::State = mem::uninitialized();
    let mut last_packet: u32 = 0;

    ignition::init(&cs, &mut state, &mut radio_state);

    io::LED_GREEN.clear(&cs);
    io::LED_YELLOW.clear(&cs);

    rfm::rfm_receive_async(11);

    loop {
        if rfm::rfm_packet_waiting() {
            radio::receive_async(&mut radio_state);
            if radio_state.valid_rx {
                last_packet = get_millis();
                state.armed = radio_state.command & (1 << 4) != 0;
                state.beep_volume = (radio_state.command >> 5) & 0x07;
                if state.armed {
                    state.fire_ch1 = radio_state.command & (1 << 0) != 0;
                    state.fire_ch2 = radio_state.command & (1 << 1) != 0;
                    state.fire_ch3 = radio_state.command & (1 << 2) != 0;
                    state.fire_ch4 = radio_state.command & (1 << 3) != 0;
                } else {
                    state.fire_ch1 = false;
                    state.fire_ch2 = false;
                    state.fire_ch3 = false;
                    state.fire_ch4 = false;
                }
            }
            delay_ms(10);
            radio::transmit(&cs, &mut state, &mut radio_state);
            rfm::rfm_receive_async(11);
            io::LED_GREEN.toggle(&cs);
            io::LED_YELLOW.toggle(&cs);
        }

        if state.armed && !radio_state.lost_link {
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

        if (get_millis() - last_packet) > PACKET_DROP_DELAY {
            radio_state.lost_link = true;
        } else {
            radio_state.lost_link = false;
        }

        if radio_state.lost_link {
            io::LED_ARM.clear(&cs);
            io::LED_DISARM.set(&cs);
            io::UPSTREAM_RELAY.clear(&cs);
            io::FIRE_CH1.clear(&cs);
            io::FIRE_CH2.clear(&cs);
            io::FIRE_CH3.clear(&cs);
            io::FIRE_CH4.clear(&cs);
        }

        beep::do_beep(&cs, &mut state);
    }
}

#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr0() {}
#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr1() {}

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn rust_eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn rust_begin_unwind(_: core::fmt::Arguments, _: &'static str, _: u32) -> ! {
    loop {}
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() {
    loop {}
}
