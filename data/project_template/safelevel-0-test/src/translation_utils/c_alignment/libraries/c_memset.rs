use crate::translation_utils::*;

pub trait CMemset<T> {
    fn c_memset(&mut self, value: T, count: usize);
}

pub fn memset(dst: &mut Ptr<u8>, value: u8, count: usize) {
    dst.c_memset(value, count)
}

impl CMemset<u8> for Ptr<u8> {
    fn c_memset(&mut self, value: u8, count: usize) {
        let mut dst_u8: Ptr<u8> = (*self).cast();
        let mut i = 0;
        while i < count {
            dst_u8[i] = value;
            i += 1;
        }
    }
}

macro_rules! c_memset {
    ($dst:expr, $src:expr, $count:expr) => {
        memset(&mut $dst.cast(), $src.cast(), $count.cast());
    };
}
pub(crate) use c_memset;
