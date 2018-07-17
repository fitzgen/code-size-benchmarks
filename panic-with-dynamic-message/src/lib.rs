use std::slice;
use std::str;

#[no_mangle]
pub unsafe extern "C" fn panic_with_dynamic_message(ptr: *const u8, len: usize) {
    let slice = slice::from_raw_parts(ptr, len);
    let msg = str::from_utf8_unchecked(slice);
    panic!("uh oh: {}", msg);
}
