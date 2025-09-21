use crate::translation_utils::*;

pub trait CMemmoveS<T> {
    fn c_memmove_s(dst: &mut Self, dst_size: usize, src: &T, count: usize) -> ErrnoT;
}

pub fn memmove_s<T2, T1: CMemmoveS<T2>>(
    dst: &mut T1,
    dst_size: usize,
    src: &T2,
    count: usize,
) -> ErrnoT {
    T1::c_memmove_s(dst, dst_size, src, count)
}

impl<T: Copy> CMemmoveS<Ptr<T>> for Ptr<T> {
    fn c_memmove_s(mut dst: &mut Ptr<T>, dst_size: usize, src: &Ptr<T>, count: usize) -> ErrnoT {
        if dst.0.is_none() || src.0.is_none() {
            return einval!();
        }
        if count as usize > dst_size {
            return erange!();
        }
        let new_count = count / core::mem::size_of::<T>();
        if *dst == *src {
            return eok!();
        }
        if *dst < *src {
            let mut i = 0;
            while i < new_count {
                dst[i] = src[i];
                i += 1;
            }
        } else {
            let mut i = new_count;
            while i > 0 {
                i -= 1;
                dst[i] = src[i];
            }
        }
        eok!()
    }
}

macro_rules! c_memmove_s {
    ($dst:expr, $dst_size:expr, $src:expr, $count:expr) => {
        memmove_s::<Ptr<u8>, Ptr<u8>>(
            &mut $dst.cast(),
            $dst_size.cast(),
            &$src.cast(),
            $count.cast(),
        )
    };
}

pub(crate) use c_memmove_s;
