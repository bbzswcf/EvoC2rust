use crate::translation_utils::*;

pub fn calloc(count: usize, size: usize) -> Ptr<u8> {
    Ptr::<u8>::c_malloc(count * size)
}
macro_rules! c_calloc {
    ($count:expr, $size:expr) => {
        calloc($count.cast(), $size.cast()).cast()
    };
}

pub(crate) use c_calloc;
