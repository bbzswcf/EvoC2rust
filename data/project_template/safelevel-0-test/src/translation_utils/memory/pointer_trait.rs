use crate::translation_utils::*;

use core::ops::*;

pub trait PointerTrait: CastFrom<Null> + Deref + PartialEq + Default {}
