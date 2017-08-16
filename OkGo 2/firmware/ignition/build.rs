extern crate gcc;

fn main() {
    gcc::Config::new()
        .target("arm-none-eabi")
        .file("src/legacy/ignition_pins.c")
        .file("src/legacy/ignition_radio.c")
        .file("src/legacy/ignition.c")
        .include("../libopencm3-sys/libopencm3/include")
        .include("../common/src/legacy")
        .include("src/legacy")
        .flag("-mthumb")
        .flag("-mcpu=cortex-m0")
        .flag("-ffreestanding")
        .define("STM32F0", None)
        .compile("libignition-legacy.a");
}
