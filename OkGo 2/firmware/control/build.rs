extern crate gcc;

fn main() {
    gcc::Config::new()
        .target("arm-none-eabi")
        .file("src/legacy/control.c")
        .file("src/legacy/control_pins.c")
        .file("src/legacy/control_radio.c")
        .file("src/legacy/display.c")
        .file("src/legacy/splash.c")
        .include("../libopencm3-sys/libopencm3/include")
        .include("../common/src/legacy")
        .include("src/legacy")
        .flag("-mthumb")
        .flag("-mcpu=cortex-m0")
        .flag("-ffreestanding")
        .define("STM32F0", None)
        .compile("libignition-legacy.a");
}
