#![no_std]
#![feature(lang_items, unwind_attributes)]
#![feature(compiler_builtins_lib)]
#![no_main]

extern crate bare_metal;
extern crate compiler_builtins;
#[macro_use]
extern crate f0;
extern crate firmware_common;
extern crate hmac;
extern crate libopencm3_sys;
extern crate md_5;
extern crate nb;
extern crate rlibc;
extern crate stm32f0xx;

mod control;
mod io;
mod lcd;
mod radio;
mod splash;

use firmware_common::utils::get_millis;

/// Max R in ohms of device otherwise CERR
const MAX_RESISTANCE: u8 = 50;
/// Delay in ms
const FAST_PACKET_DELAY: u32 = 250;
/// Delay in ms
const SLOW_PACKET_DELAY: u32 = 1000;
/// Delay in ms
const PACKET_DROP_DELAY: u32 = 2000;

#[no_mangle]
pub unsafe extern "C" fn main() -> i32 {
    return rs_main();
}

fn rs_main() -> i32 {
    let cs = unsafe { bare_metal::CriticalSection::new() };
    {
        let cs = &cs;
        let mut last_packet: u32 = 0;
        let mut lost_link;

        let beep_volume;
        if !io::SW_CH4.get_bool(cs) {
            beep_volume = 3; // high
        } else if !io::SW_CH3.get_bool(cs) {
            beep_volume = 2; // med
        } else if !io::SW_CH2.get_bool(cs) {
            beep_volume = 1; // low
        } else if !io::SW_CH1.get_bool(cs) {
            beep_volume = 0; // off
        } else {
            // Default: deafen
            beep_volume = 3;
        }

        let mut state = control::State {
            armed: false,
            beep_volume: beep_volume,
            centre_frf: firmware_common::rfm::Frf::Frf868 as u32,
            ch1_status: control::ChannelStatus::Ok,
            ch2_status: control::ChannelStatus::Ok,
            ch3_status: control::ChannelStatus::Ok,
            ch4_status: control::ChannelStatus::Ok,
        };

        let mut radio_state = radio::State {
            rx_rssi: 0,
            rx_voltage: 0,
            rx_status: 0,
            rx_cont1: 0,
            rx_cont2: 0,
            rx_cont3: 0,
            rx_cont4: 0,
            packet_rssi: 0,
        };

        control::init(cs);

        splash::splash(cs);

        io::LED_GREEN.set(cs);
        io::LED_YELLOW.clear(cs);

        loop {
            let loop_timer = get_millis();

            match radio::receive_async() {
                Ok(new_state) => {
                    radio_state = new_state;
                    last_packet = loop_timer;
                }
                _ => (),
            }

            if (loop_timer - last_packet) > PACKET_DROP_DELAY {
                lost_link = true;
            } else {
                lost_link = false;
            }

            // Arming status
            state.armed = !io::SW_KEY.get_bool(cs);
            io::LED_ARM.set_bool(cs, state.armed);
            io::LED_DISARM.set_bool(cs, !state.armed);

            // Start with channels okay and un-firing
            state.ch1_status = control::ChannelStatus::Ok;
            state.ch2_status = control::ChannelStatus::Ok;
            state.ch3_status = control::ChannelStatus::Ok;
            state.ch4_status = control::ChannelStatus::Ok;

            // Detect continuity errors
            if !lost_link {
                if radio_state.rx_cont1 > MAX_RESISTANCE {
                    state.ch1_status = control::ChannelStatus::ContinuityError;
                }
                if radio_state.rx_cont2 > MAX_RESISTANCE {
                    state.ch2_status = control::ChannelStatus::ContinuityError;
                }
                if radio_state.rx_cont3 > MAX_RESISTANCE {
                    state.ch3_status = control::ChannelStatus::ContinuityError;
                }
                if radio_state.rx_cont4 > MAX_RESISTANCE {
                    state.ch4_status = control::ChannelStatus::ContinuityError;
                }
            }

            // Detect firing state
            if state.armed {
                if !io::SW_CH1.get_bool(cs) {
                    state.ch1_status = control::ChannelStatus::Fire;
                }
                if !io::SW_CH2.get_bool(cs) {
                    state.ch2_status = control::ChannelStatus::Fire;
                }
                if !io::SW_CH3.get_bool(cs) {
                    state.ch3_status = control::ChannelStatus::Fire;
                }
                if !io::SW_CH4.get_bool(cs) {
                    state.ch4_status = control::ChannelStatus::Fire;
                }
            }

            // Set channel LEDs based on firing/cont and armed status
            io::LED_CH1.set_bool(
                cs,
                state.armed && (state.ch1_status == control::ChannelStatus::Ok),
            );
            io::LED_CH2.set_bool(
                cs,
                state.armed && (state.ch2_status == control::ChannelStatus::Ok),
            );
            io::LED_CH3.set_bool(
                cs,
                state.armed && (state.ch3_status == control::ChannelStatus::Ok),
            );
            io::LED_CH4.set_bool(
                cs,
                state.armed && (state.ch4_status == control::ChannelStatus::Ok),
            );

            // Do TX
            io::LED_YELLOW.set(cs);
            radio::transmit(&state);
            io::LED_YELLOW.clear(cs);

            // Setup receiver
            radio::make_ready();

            // Update display
            control::display_update(cs, &state, &radio_state, lost_link);

            // Delay to correctly set loop timing
            let packet_delay = if state.armed {
                FAST_PACKET_DELAY
            } else {
                SLOW_PACKET_DELAY
            };
            while (get_millis() - loop_timer) < packet_delay {}

            io::LED_GREEN.toggle(cs);
        }
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
