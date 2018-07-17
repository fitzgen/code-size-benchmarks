extern crate regex;

use std::slice;
use std::str;

#[no_mangle]
pub unsafe extern "C" fn does_it_match(
    re_ptr: *const u8,
    re_len: usize,
    input_ptr: *const u8,
    input_len: usize,
) -> bool {
    let re = slice::from_raw_parts(re_ptr, re_len);
    let re = str::from_utf8_unchecked(re);
    let re = regex::Regex::new(re).unwrap();

    let input = slice::from_raw_parts(input_ptr, input_len);
    let input = str::from_utf8_unchecked(input);

    re.is_match(input)
}
