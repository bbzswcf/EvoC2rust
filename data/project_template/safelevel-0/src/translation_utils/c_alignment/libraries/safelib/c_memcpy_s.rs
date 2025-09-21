use crate::translation_utils::*;

pub trait CMemcpyS<T> {
    fn c_memcpy_s(dst: &mut Self, dst_size: usize, src: &T, count: usize) -> ErrnoT;
}

pub fn memcpy_s<T2, T1: CMemcpyS<T2>>(
    dst: &mut T1,
    dst_size: usize,
    src: &T2,
    count: usize,
) -> ErrnoT {
    T1::c_memcpy_s(dst, dst_size, src, count)
}

impl<T: Copy> CMemcpyS<Ptr<T>> for Ptr<T> {
    fn c_memcpy_s(mut dst: &mut Ptr<T>, dst_size: usize, src: &Ptr<T>, count: usize) -> ErrnoT {
        if dst.0.is_none() || src.0.is_none() {
            return einval!();
        }
        if count as usize > dst_size {
            return erange!();
        }
        let new_count = count / core::mem::size_of::<T>();
        let mut i = 0;
        while i < new_count {
            dst[i] = src[i];
            i += 1;
        }
        eok!()
    }
}

macro_rules! c_memcpy_s {
    ($dst:expr, $dst_size:expr, $src:expr, $count:expr) => {
        memcpy_s::<Ptr<u8>, Ptr<u8>>(
            &mut $dst.cast(),
            $dst_size.cast(),
            &$src.cast(),
            $count.cast(),
        )
    };
}

pub(crate) use c_memcpy_s;
