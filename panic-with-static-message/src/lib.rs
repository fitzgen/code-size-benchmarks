#[no_mangle]
pub unsafe extern "C" fn panic_with_static_message() {
    panic!("ground control to major tom")
}
