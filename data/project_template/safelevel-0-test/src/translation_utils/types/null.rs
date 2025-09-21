use crate::translation_utils::*;

use core::ops::*;

pub struct Null();

macro_rules! null {
    () => {
        Null().cast()
    };
}

macro_rules! NULL {
    () => {
        Null().cast()
    };
}

impl CastFrom<Null> for Null {
    fn cast_from(_: &mut Null) -> Self {
        Null()
    }
}

pub(crate) use null;
pub(crate) use NULL;
