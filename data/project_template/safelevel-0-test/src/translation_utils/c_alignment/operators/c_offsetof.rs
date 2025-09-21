use crate::translation_utils::*;

macro_rules! c_offsetof {
    ($Container:ty, $($fields:tt).+ $(,)?) => {
        core::mem::offset_of!($Container, $($fields)+)
    };
}
pub(crate) use c_offsetof;
