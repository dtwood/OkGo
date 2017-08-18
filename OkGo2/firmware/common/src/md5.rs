use core::ptr::copy_nonoverlapping;
use core::slice;

use md_5::{Digest, Md5};

/// Produces the MD5 digest of the message stored at *message (length
/// message_len) and places the result in the 16-byte buffer at *hash
#[no_mangle]
pub unsafe extern "C" fn md5(message: *const u8, message_len: usize, hash: *mut u8) {
    let message = slice::from_raw_parts(message, message_len);
    let h = Md5::digest(message);
    copy_nonoverlapping(h.as_ptr(), hash, 16);
}
