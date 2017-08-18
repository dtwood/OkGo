use core::slice;

extern "C" {
    pub static key: u8;
    pub static key_len: u8;
}

pub fn get_key() -> &'static [u8] {
    unsafe { &slice::from_raw_parts(&key, key_len.into()) }
}
