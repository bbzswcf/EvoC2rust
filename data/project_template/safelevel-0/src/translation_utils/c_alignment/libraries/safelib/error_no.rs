use crate::translation_utils::*;

pub type errno_t = i32;

pub type ErrnoT = i32;

macro_rules! eok {
    () => {
        0
    };
}
macro_rules! einval {
    () => {
        22
    };
}
macro_rules! erange {
    () => {
        34
    };
}

pub(crate) use einval;
pub(crate) use eok;
pub(crate) use erange;

macro_rules! EOK {
    () => {
        0
    };
}
macro_rules! EINVAL {
    () => {
        22
    };
}
macro_rules! ERANGE {
    () => {
        34
    };
}

pub(crate) use EINVAL;
pub(crate) use EOK;
pub(crate) use ERANGE;
