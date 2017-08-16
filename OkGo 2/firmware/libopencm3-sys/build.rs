extern crate make;

fn main() {
    let dst = make::Config::new("libopencm3")
        .env("DEBUG_FLAGS", "-ffreestanding")
        .build();
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=opencm3_stm32f0");
    println!("cargo:rustc-link-lib=gcc");
}
