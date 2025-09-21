use crate::translation_utils::*;

pub trait CMemsetS<T> {
    fn c_memset_s(dst: Self, count: usize, value: T, value_size: usize) -> ErrnoT;
}

pub fn memset_s<T2, T1: CMemsetS<T2>>(
    dst: T1,
    count: usize,
    value: T2,
    value_size: usize,
) -> ErrnoT {
    T1::c_memset_s(dst, count, value, value_size)
}

impl<T> CMemsetS<u8> for Ptr<T> {
    fn c_memset_s(mut dst: Ptr<T>, count: usize, value: u8, value_size: usize) -> ErrnoT {
        if dst.0.is_none() {
            return einval!();
        }
        if value_size > count {
            return einval!();
        }
        if count as usize != 0 {
            let mut dst_u8: Ptr<u8> = dst.cast();
            let mut i = 0;
            while i < value_size {
                dst_u8[i] = value;
                i += 1;
            }
        }
        eok!()
    }
}

macro_rules! c_memset_s {
    ($dst:expr, $count:expr, $value:expr, $value_size:expr) => {
        memset_s(
            $dst.cast::<Ptr<u8>>(),
            $count.cast(),
            $value.cast(),
            $value_size.cast(),
        )
    };
}
pub(crate) use c_memset_s;
