#![no_std]
#![feature(lang_items, unwind_attributes)]
#![feature(compiler_builtins_lib)]
#![no_main]

extern crate libopencm3_sys;
extern crate firmware_common;
extern crate rlibc;
extern crate compiler_builtins;
extern crate f0;
extern crate bare_metal;
extern crate stm32f0xx;
extern crate hmac;
extern crate md_5;

pub mod beep;
pub mod radio;
pub mod ignition;
pub mod io;

extern "C" {
    fn c_main() -> i32;
}

#[no_mangle]
pub unsafe extern "C" fn main() -> i32 {
    return c_main();
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
