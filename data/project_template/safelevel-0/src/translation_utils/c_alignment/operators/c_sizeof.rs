use crate::translation_utils::*;

macro_rules! c_sizeof {
    ($t:ty) => {
        Num(core::mem::size_of::<$t>()) 
    };
}

macro_rules! c_sizeofval {
    ($v:expr) => {
        Num(core::mem::size_of_val(&$v))
    };
}

pub(crate) use c_sizeof;
pub(crate) use c_sizeofval;
