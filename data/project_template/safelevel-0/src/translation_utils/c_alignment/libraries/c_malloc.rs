use crate::translation_utils::*;

pub trait CMalloc {
    fn c_malloc(size: usize) -> Self;
}

pub fn malloc(size: usize) -> Ptr<u8> {
    Ptr::<u8>::c_malloc(size)
}

impl CMalloc for Ptr<u8> {
    fn c_malloc(size: usize) -> Self {
        if size == 0 {
            return null!();
        }
        unsafe { RESOURCE_HEAP.alloc(size).cast() }
    }
}

macro_rules! c_malloc {
    ($size:expr) => {
        malloc($size.cast()).cast()
    };
}

pub(crate) use c_malloc;
