extern crate gcc;

fn main() {
    gcc::Config::new()
        .target("arm-none-eabi")
        .file("src/legacy/rfm95w.c")
        .file("src/legacy/utils.c")
        .file("src/legacy/key.c")
        .include("../libopencm3-sys/libopencm3/include")
        .include("src/legacy")
        .flag("-mthumb")
        .flag("-mcpu=cortex-m0")
        .define("STM32F0", None)
        .compile("libfirmwarecommon-legacy.a");
}
