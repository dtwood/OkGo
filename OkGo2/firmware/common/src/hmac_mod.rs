use core::ptr::copy_nonoverlapping;
use core::slice;

use hmac::{Hmac, Mac};
use md5::Md5;

/// Generate an HMAC-MD5 signature for *message, length message_len, with
/// key *key, length key_len, and store the result in the 16-byte buffer at
/// *hash
#[no_mangle]
pub unsafe extern "C" fn hmac_md5(
    message: *const u8,
    message_len: u8,
    key: *const u8,
    key_len: u8,
    hash: *mut u8,
) {
    let mut mac = Hmac::<Md5>::new(slice::from_raw_parts(key, key_len.into())).unwrap();
    mac.input(slice::from_raw_parts(message, message_len.into()));

    let result = mac.result();
    copy_nonoverlapping(result.code().as_ptr(), hash, 16);
}

/// Generate a truncated HMAC-MD5-80 signature for *message, length message_len,
/// with key *key, length key_len, and store the result in the 10-byte buffer at
/// *hash */
#[no_mangle]
pub unsafe extern "C" fn hmac_md5_80(
    message: *const u8,
    message_len: u8,
    key: *const u8,
    key_len: u8,
    hash: *mut u8,
) {
    let mut mac = Hmac::<Md5>::new(slice::from_raw_parts(key, key_len.into())).unwrap();
    mac.input(slice::from_raw_parts(message, message_len.into()));

    let result = mac.result();
    copy_nonoverlapping(result.code().as_ptr(), hash, 10);
}
