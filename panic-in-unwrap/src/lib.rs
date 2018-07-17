#[no_mangle]
pub unsafe extern "C" fn panic_in_bounds_check(ptr: *const u8) -> u8 {
    let ptr = ptr as *const Result<u8, ()>;
    let result = *ptr;
    result.unwrap()
}
