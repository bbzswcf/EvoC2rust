use crate::translation_utils::*;

pub trait CMemcpy<T> {
    fn c_memcpy(&mut self, src: &T, count: usize);
}

pub fn memcpy(dst: &mut Ptr<u8>, src: &Ptr<u8>, count: usize) {
    dst.c_memcpy(src, count)
}

impl CMemcpy<Ptr<u8>> for Ptr<u8> {
    fn c_memcpy(&mut self, src: &Ptr<u8>, count: usize) {
        let new_count = count;
        let mut i = 0;
        while i < new_count {
            self[i] = src[i];
            i += 1;
        }
    }
}
macro_rules! c_memcpy {
    ($dst:expr, $src:expr, $count:expr) => {
        if $count as usize != 0 {
            memcpy(&mut $dst.cast(), &$src.cast(), $count.cast());
        }
    };
}

pub(crate) use c_memcpy;
