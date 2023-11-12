use core::ptr;
use Allocator;

pub struct System;

impl System {
    pub const fn new() -> System {
        System
    }
}

unsafe impl Allocator for System {
    fn alloc(&self, mut size: usize) -> (*mut u8, usize, u32) {
        let real_size = size;
        core::arch::asm!(
            "li x0, 7",
            inout("a0") size,
            options(pure, nomem, nostack)
        );
        if size == u32::MAX as usize {
            return (ptr::null_mut(), 0, 0);
        }
        (size as *mut u8, real_size, 0)
    }

    fn remap(&self, _ptr: *mut u8, _oldsize: usize, _newsize: usize, _can_move: bool) -> *mut u8 {
        ptr::null_mut()
    }

    fn free_part(&self, _ptr: *mut u8, _oldsize: usize, _newsize: usize) -> bool {
        false
    }

    fn free(&self, _ptr: *mut u8, _size: usize) -> bool {
        false
    }

    fn can_release_part(&self, _flags: u32) -> bool {
        false
    }

    fn allocates_zeros(&self) -> bool {
        true
    }

    fn page_size(&self) -> usize {
        64 * 1024
    }
}
