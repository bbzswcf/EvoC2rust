use crate::translation_utils::*;

use core::ops::*;

#[derive(Clone, Copy)]
pub struct FuncPtr<T>(pub Option<T>)
where
    T: Copy + Eq;

impl<T: Copy + Eq> FuncPtr<T> {
    pub const fn new(value: T) -> Self {
        FuncPtr(Some(value))
    }
}

impl<T: Copy + Eq> PointerTrait for FuncPtr<T> {}

impl<T: Copy + Eq> Default for FuncPtr<T> {
    fn default() -> Self {
        FuncPtr(None)
    }
}

impl<T: Copy + Eq> CastFrom<Null> for FuncPtr<T> {
    fn cast_from(_: &mut Null) -> Self {
        FuncPtr(None)
    }
}

impl<T: Copy + Eq> PartialEq for FuncPtr<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.0.is_none() && other.0.is_none() {
            return true;
        } else if self.0.is_none() || other.0.is_none() {
            return false;
        } else {
            return self.0.unwrap() == other.0.unwrap();
        }
    }
}

impl<T: Copy + Eq> Deref for FuncPtr<T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.0.as_ref().unwrap()
    }
}

impl<T: Copy + Eq> DerefMut for FuncPtr<T> {
    fn deref_mut(&mut self) -> &mut T {
        self.0.as_mut().unwrap()
    }
}

impl<T: Copy + Eq> CastFrom<FuncPtr<T>> for FuncPtr<T> {
    fn cast_from(ptr: &mut FuncPtr<T>) -> Self {
        *ptr
    }
}

macro_rules! func {
    ($name:expr) => {
        FuncPtr::new($name)
    };
}

pub(crate) use func;
