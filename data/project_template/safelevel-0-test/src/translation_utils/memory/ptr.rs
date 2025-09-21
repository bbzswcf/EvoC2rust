use crate::translation_utils::*;

use core::ops::*;

use core::cell::UnsafeCell;
use core::ptr::NonNull;

pub struct Ptr<T>(pub Option<NonNull<T>>);

unsafe impl<T> Send for Ptr<T> {}

impl<T: std::fmt::Display> std::fmt::Debug for Ptr<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> std::fmt::Result {
        if self.0.is_none() {
            write!(f, "Ptr(None)")
        } else {
            write!(f, "Ptr(Some(&{}))", unsafe { self.0.unwrap().as_ref() })
        }
    }
}

impl<T> Default for Ptr<T> {
    fn default() -> Self {
        Ptr(None)
    }
}

impl<T> Ptr<T> {
    pub fn new(value: &mut T) -> Self {
        Ptr(Some(NonNull::new(value as *mut T).unwrap()))
    }

    pub fn as_bool(&self) -> bool {
        self.0.is_some()
    }
}

impl<T> Clone for Ptr<T> {
    fn clone(&self) -> Self {
        Ptr(self.0)
    }
}

impl<T> PointerTrait for Ptr<T> {}

impl<T> CastFrom<Null> for Ptr<T> {
    fn cast_from(_: &mut Null) -> Ptr<T> {
        Ptr(None)
    }
}

impl<T1, T2> CastFrom<Ptr<T1>> for Ptr<T2> {
    fn cast_from(ptr: &mut Ptr<T1>) -> Ptr<T2> {
        Ptr(ptr.0.map(|ptr| ptr.cast()))
    }
}

impl<T, I: Integer> CastFrom<Ptr<T>> for I {
    fn cast_from(ptr: &mut Ptr<T>) -> I {
        I::from_usize(ptr.0.unwrap().as_ptr() as usize)
    }
}

impl<T1, T2, const N: usize> CastFrom<Array<T2, N>> for Ptr<T1> {
    fn cast_from(array: &mut Array<T2, N>) -> Ptr<T1> {
        Ptr(Some(NonNull::new(array.0.as_mut_ptr() as *mut T1).unwrap()))
    }
}

impl<T> Copy for Ptr<T> {}

impl<T> PartialEq for Ptr<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> PartialOrd for Ptr<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T> Deref for Ptr<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { self.0.unwrap().as_ref() }
    }
}

impl<T> DerefMut for Ptr<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { self.0.unwrap().as_mut() }
    }
}

impl<T, I: Integer> Index<I> for Ptr<T> {
    type Output = T;
    fn index(&self, index: I) -> &T {
        unsafe {
            self.0
                .unwrap()
                .as_ptr()
                .add(index.as_usize())
                .as_ref()
                .unwrap()
        }
    }
}

impl<T, I: Integer> IndexMut<I> for Ptr<T> {
    fn index_mut(&mut self, index: I) -> &mut T {
        unsafe {
            self.0
                .unwrap()
                .as_ptr()
                .add(index.as_usize())
                .as_mut()
                .unwrap()
        }
    }
}

impl<T, I: Integer> Add<I> for Ptr<T> {
    type Output = Ptr<T>;
    fn add(self, rhs: I) -> Ptr<T> {
        Ptr(self
            .0
            .map(|ptr| unsafe { NonNull::new_unchecked(ptr.as_ptr().add(rhs.as_usize())) }))
    }
}

impl<T, I: Integer> AddAssign<I> for Ptr<T> {
    fn add_assign(&mut self, rhs: I) {
        self.0 = self
            .0
            .map(|ptr| unsafe { NonNull::new_unchecked(ptr.as_ptr().add(rhs.as_usize())) });
    }
}

impl<T, I: Integer> Sub<I> for Ptr<T> {
    type Output = Ptr<T>;
    fn sub(self, rhs: I) -> Ptr<T> {
        Ptr(self
            .0
            .map(|ptr| unsafe { NonNull::new_unchecked(ptr.as_ptr().sub(rhs.as_usize())) }))
    }
}

impl<T, I: Integer> SubAssign<I> for Ptr<T> {
    fn sub_assign(&mut self, rhs: I) {
        self.0 = self
            .0
            .map(|ptr| unsafe { NonNull::new_unchecked(ptr.as_ptr().sub(rhs.as_usize())) });
    }
}

impl<T> Sub<Ptr<T>> for Ptr<T> {
    type Output = usize;
    fn sub(self, rhs: Ptr<T>) -> usize {
        (self.0.unwrap().as_ptr() as usize - rhs.0.unwrap().as_ptr() as usize)
            / core::mem::size_of::<T>()
    }
}
