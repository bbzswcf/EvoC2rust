use crate::translation_utils::*;

pub trait CMemmove<T> {
    fn c_memmove(&mut self, src: &T, count: usize);
}

pub fn memmove(dst: &mut Ptr<u8>, src: &Ptr<u8>, count: usize) {
    dst.c_memmove(src, count)
}

impl CMemmove<Ptr<u8>> for Ptr<u8> {
    fn c_memmove(&mut self, src: &Ptr<u8>, count: usize) {
        let new_count = count;
        if *self == *src {
            return;
        }
        if *self < *src {
            let mut i = 0;
            while i < new_count {
                self[i] = src[i];
                i += 1;
            }
        } else {
            let mut i = new_count;
            while i > 0 {
                i -= 1;
                self[i] = src[i];
            }
        }
    }
}

macro_rules! c_memmove {
    ($dst:expr, $src:expr, $count:expr) => {
        if $count as usize != 0 {
            memmove(&mut $dst.cast(), &$src.cast(), $count.cast());
        }
    };
}

pub(crate) use c_memmove;
