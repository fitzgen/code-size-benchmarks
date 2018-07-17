extern crate wee_alloc;

use std::alloc::{GlobalAlloc, Layout};
use std::ptr;

static ALLOCATOR: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[no_mangle]
pub unsafe extern "C" fn alloc(size: usize, align: usize) -> *mut u8 {
    Layout::from_size_align(size, align)
        .ok()
        .map_or(ptr::null_mut(), |layout| ALLOCATOR.alloc(layout))
}

#[no_mangle]
pub unsafe fn alloc_zeroed(size: usize, align: usize) -> *mut u8 {
    Layout::from_size_align(size, align)
        .ok()
        .map_or(ptr::null_mut(), |layout| ALLOCATOR.alloc_zeroed(layout))
}

#[no_mangle]
pub unsafe extern "C" fn dealloc(size: usize, align: usize, ptr: *mut u8) {
    if let Ok(layout) = Layout::from_size_align(size, align) {
        ALLOCATOR.dealloc(ptr, layout)
    }
}

#[no_mangle]
pub unsafe fn realloc(ptr: *mut u8, old_size: usize, align: usize, new_size: usize) -> *mut u8 {
    Layout::from_size_align(old_size, align)
        .ok()
        .map_or(ptr::null_mut(), |layout| {
            ALLOCATOR.realloc(ptr, layout, new_size)
        })
}
