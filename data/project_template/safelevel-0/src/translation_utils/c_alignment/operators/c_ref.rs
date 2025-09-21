use crate::translation_utils::*;

use core::ptr::NonNull;

pub trait CRef {
    type Target;
    fn c_ref(&mut self) -> Self::Target;
}

impl<T> CRef for T {
    type Target = Ptr<T>;
    fn c_ref(&mut self) -> Ptr<T> {
        Ptr::new(self)
    }
}

macro_rules! c_ref {
    ($ptr:expr) => {
        ($ptr).c_ref()
    };
}

pub(crate) use c_ref;
