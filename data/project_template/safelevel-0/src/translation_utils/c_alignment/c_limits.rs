use crate::translation_utils::*;

macro_rules! SCHAR_MIN {
    () => {
        i8::MIN
    };
}
pub(crate) use SCHAR_MIN;

macro_rules! SCHAR_MAX {
    () => {
        i8::MAX
    };
}
pub(crate) use SCHAR_MAX;

macro_rules! UCHAR_MAX {
    () => {
        u8::MAX
    };
}
pub(crate) use UCHAR_MAX;

macro_rules! SHRT_MIN {
    () => {
        i16::MIN
    };
}
pub(crate) use SHRT_MIN;

macro_rules! SHRT_MAX {
    () => {
        i16::MAX
    };
}
pub(crate) use SHRT_MAX;

macro_rules! USHRT_MAX {
    () => {
        u16::MAX
    };
}
pub(crate) use USHRT_MAX;

macro_rules! INT_MIN {
    () => {
        i32::MIN
    };
}
pub(crate) use INT_MIN;

macro_rules! INT_MAX {
    () => {
        i32::MAX
    };
}
pub(crate) use INT_MAX;

macro_rules! UINT_MAX {
    () => {
        u32::MAX
    };
}
pub(crate) use UINT_MAX;

macro_rules! LLONG_MIN {
    () => {
        i64::MIN
    };
}
pub(crate) use LLONG_MIN;

macro_rules! LLONG_MAX {
    () => {
        i64::MAX
    };
}
pub(crate) use LLONG_MAX;

macro_rules! ULLONG_MAX {
    () => {
        u64::MAX
    };
}
pub(crate) use ULLONG_MAX;



