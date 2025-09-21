use crate::translation_utils::*;

use core::ops::*;

pub struct Array<T, const N: usize>(pub [T; N]);

use core::ptr::NonNull;

impl<T, const N: usize> Array<T, N> {
    pub fn new() -> Self
    where
        T: Default,
    {
        Array([(); N].map(|_| Default::default()))
    }

    pub const fn len(&self) -> usize {
        N
    }

    pub const fn from(data: [T; N]) -> Array<T, N> {
        Array(data)
    }

    pub fn as_rust_slice(&self) -> &[T] {
        &self.0
    }

    pub fn as_rust_slice_mut(&mut self) -> &mut [T] {
        &mut self.0
    }
}

impl<T: Default, const N: usize> Default for Array<T, N> {
    fn default() -> Self {
        Array([(); N].map(|_| Default::default()))
    }
}

impl<T, const N: usize, I: Integer> Index<I> for Array<T, N> {
    type Output = T;
    fn index(&self, index: I) -> &T {
        &self.as_rust_slice()[index.as_usize()]
    }
}

impl<T, const N: usize, I: Integer> IndexMut<I> for Array<T, N> {
    fn index_mut(&mut self, index: I) -> &mut T {
        &mut self.as_rust_slice_mut()[index.as_usize()]
    }
}

impl<T, const N: usize> Index<Range<usize>> for Array<T, N> {
    type Output = [T];
    fn index(&self, index: Range<usize>) -> &[T] {
        &self.as_rust_slice()[index]
    }
}

impl<T, const N: usize> IndexMut<Range<usize>> for Array<T, N> {
    fn index_mut(&mut self, index: Range<usize>) -> &mut [T] {
        &mut self.as_rust_slice_mut()[index]
    }
}

impl<T, const N: usize> Index<RangeTo<usize>> for Array<T, N> {
    type Output = [T];
    fn index(&self, index: RangeTo<usize>) -> &[T] {
        &self.as_rust_slice()[index]
    }
}

impl<T, const N: usize> IndexMut<RangeTo<usize>> for Array<T, N> {
    fn index_mut(&mut self, index: RangeTo<usize>) -> &mut [T] {
        &mut self.as_rust_slice_mut()[index]
    }
}

impl<T, const N: usize> Index<RangeFrom<usize>> for Array<T, N> {
    type Output = [T];
    fn index(&self, index: RangeFrom<usize>) -> &[T] {
        &self.as_rust_slice()[index]
    }
}

impl<T, const N: usize> IndexMut<RangeFrom<usize>> for Array<T, N> {
    fn index_mut(&mut self, index: RangeFrom<usize>) -> &mut [T] {
        &mut self.as_rust_slice_mut()[index]
    }
}

impl<T, const N: usize> Index<RangeFull> for Array<T, N> {
    type Output = [T];
    fn index(&self, index: RangeFull) -> &[T] {
        &self.as_rust_slice()[index]
    }
}

impl<T, const N: usize> IndexMut<RangeFull> for Array<T, N> {
    fn index_mut(&mut self, index: RangeFull) -> &mut [T] {
        &mut self.as_rust_slice_mut()[index]
    }
}

impl<T, const N: usize> Index<RangeInclusive<usize>> for Array<T, N> {
    type Output = [T];
    fn index(&self, index: RangeInclusive<usize>) -> &[T] {
        &self.as_rust_slice()[index]
    }
}

impl<T, const N: usize> IndexMut<RangeInclusive<usize>> for Array<T, N> {
    fn index_mut(&mut self, index: RangeInclusive<usize>) -> &mut [T] {
        &mut self.as_rust_slice_mut()[index]
    }
}

impl<T, const N: usize> Index<RangeToInclusive<usize>> for Array<T, N> {
    type Output = [T];
    fn index(&self, index: RangeToInclusive<usize>) -> &[T] {
        &self.as_rust_slice()[index]
    }
}

impl<T, const N: usize> IndexMut<RangeToInclusive<usize>> for Array<T, N> {
    fn index_mut(&mut self, index: RangeToInclusive<usize>) -> &mut [T] {
        &mut self.as_rust_slice_mut()[index]
    }
}

macro_rules! arr {
    // from [1, 2, 3, 4]
    ($($x:expr),+ $(,)?) => {
        Array::from([$($x),+])
    };
    // from [1; N]
    ($x:expr; $size:expr) => {
        Array::from([$x; $size])
    };
    // from []
    () => {
        Array::from([])
    };
}

pub(crate) use arr;

macro_rules! index {
    ($array: expr, $index: expr) => {
        let __tmp = $index;
        $array[__tmp]
    };
    ($array: expr, $index: expr, $value: expr) => {
        let __tmp = $index;
        $array[__tmp] = $value;
    };
}

pub(crate) use index;
