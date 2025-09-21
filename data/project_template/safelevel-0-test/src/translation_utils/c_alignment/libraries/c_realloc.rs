use crate::translation_utils::*;

pub trait CRealloc {
    fn c_realloc(&mut self, size: usize) -> Self;
}

pub fn realloc(mut ptr: Ptr<u8>, mut size: usize) -> Ptr<u8> {
    ptr.c_realloc(size)
}

impl CRealloc for Ptr<u8> {
    fn c_realloc(&mut self, mut size: usize) -> Self {
        if size == 0 {
            c_free!(self);
            return null!();
        } else {
            if self.0.is_none() {
                return c_malloc!(size);
            } else {
                let new_ptr = unsafe { RESOURCE_HEAP.realloc(self.cast(), size).cast() };
                return new_ptr;
            }
        }
    }
}

macro_rules! c_realloc {
    ($ptr:expr, $size:expr) => {
        realloc($ptr.cast(), $size.cast()).cast()
    };
}

pub(crate) use c_realloc;
