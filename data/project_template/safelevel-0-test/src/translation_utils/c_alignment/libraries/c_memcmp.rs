use crate::translation_utils::*;

use core::cmp::Ordering;

pub trait CMemcmp<T> {
    fn c_memcmp(dst: &Self, src: &T, count: usize) -> i32;
}

pub fn memcmp<T2, T1: CMemcmp<T2>>(dst: &T1, src: &T2, count: usize) -> i32 {
    T1::c_memcmp(dst, src, count)
}

impl<T: std::cmp::PartialOrd> CMemcmp<Ptr<T>> for Ptr<T> {
    fn c_memcmp(mut dst: &Ptr<T>, mut src: &Ptr<T>, count: usize) -> i32 {
        let new_count = count / core::mem::size_of::<T>();
        let mut i = 0;
        while i < new_count {
            if dst[i] > src[i] {
                return 1;
            } else if dst[i] < src[i] {
                return -1;
            }
            i += 1;
        }
        0
    }
}

macro_rules! c_memcmp {
    ($dst:expr, $src:expr, $count:expr) => {
        memcmp::<Ptr<u8>, Ptr<u8>>(&$dst.cast(), &$src.cast(), $count.cast())
    };
}

pub(crate) use c_memcmp;
