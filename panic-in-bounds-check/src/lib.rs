use std::slice;
use std::str;

#[no_mangle]
pub unsafe extern "C" fn panic_in_bounds_check(length_prefixed: *const u8, idx: usize) -> u8 {
    let len = *(length_prefixed as *const u32) as usize;
    let ptr = length_prefixed.offset(4);
    let slice = slice::from_raw_parts(ptr, len);
    slice[idx]
}
