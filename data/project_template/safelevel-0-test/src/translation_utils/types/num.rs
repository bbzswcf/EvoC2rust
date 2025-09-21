use crate::translation_utils::*;

use core::ops::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Num(pub usize);

impl PrimitiveType for Num {}

impl Integer for Num {
    fn as_bool(self) -> bool {
        self.0 != 0
    }

    fn as_usize(self) -> usize {
        self.0
    }

    fn from_usize(value: usize) -> Self {
        Num(value)
    }
}

impl <I: Integer> Add<I> for Num {
    type Output = I;

    fn add(self, rhs: I) -> Self::Output {
        I::from_usize(self.0 + rhs.as_usize())
    }
}

impl <I: Integer> Sub<I> for Num {
    type Output = I;

    fn sub(self, rhs: I) -> Self::Output {
        I::from_usize(self.0 - rhs.as_usize())
    }
}

impl <I: Integer> Mul<I> for Num {
    type Output = I;

    fn mul(self, rhs: I) -> Self::Output {
        I::from_usize(self.0 * rhs.as_usize())
    }
}

impl <I: Integer> Div<I> for Num {
    type Output = I;

    fn div(self, rhs: I) -> Self::Output {
        I::from_usize(self.0 / rhs.as_usize())
    }
}

impl Add<Num> for u8 {
    type Output = u8;

    fn add(self, rhs: Num) -> Self::Output {
        self + rhs.0 as u8
    }
}

impl Add<Num> for u16 {
    type Output = u16;

    fn add(self, rhs: Num) -> Self::Output {
        self + rhs.0 as u16
    }
}

impl Add<Num> for u32 {
    type Output = u32;

    fn add(self, rhs: Num) -> Self::Output {
        self + rhs.0 as u32
    }
}

impl Add<Num> for u64 {
    type Output = u64;

    fn add(self, rhs: Num) -> Self::Output {
        self + rhs.0 as u64
    }
}

impl Add<Num> for usize {
    type Output = usize;

    fn add(self, rhs: Num) -> Self::Output {
        self + rhs.0
    }
}

impl Add<Num> for i8 {
    type Output = i8;

    fn add(self, rhs: Num) -> Self::Output {
        self + rhs.0 as i8
    }
}

impl Add<Num> for i16 {
    type Output = i16;

    fn add(self, rhs: Num) -> Self::Output {
        self + rhs.0 as i16
    }
}

impl Add<Num> for i32 {
    type Output = i32;

    fn add(self, rhs: Num) -> Self::Output {
        self + rhs.0 as i32
    }
}

impl Add<Num> for i64 {
    type Output = i64;

    fn add(self, rhs: Num) -> Self::Output {
        self + rhs.0 as i64
    }
}

impl Add<Num> for isize {
    type Output = isize;

    fn add(self, rhs: Num) -> Self::Output {
        self + rhs.0 as isize
    }
}

impl Mul<Num> for u8 {
    type Output = u8;

    fn mul(self, rhs: Num) -> Self::Output {
        self * rhs.0 as u8
    }
}

impl Mul<Num> for u16 {
    type Output = u16;

    fn mul(self, rhs: Num) -> Self::Output {
        self * rhs.0 as u16
    }
}

impl Mul<Num> for u32 {
    type Output = u32;

    fn mul(self, rhs: Num) -> Self::Output {
        self * rhs.0 as u32
    }
}

impl Mul<Num> for u64 {
    type Output = u64;

    fn mul(self, rhs: Num) -> Self::Output {
        self * rhs.0 as u64
    }
}

impl Mul<Num> for usize {
    type Output = usize;

    fn mul(self, rhs: Num) -> Self::Output {
        self * rhs.0
    }
}

impl Mul<Num> for i8 {
    type Output = i8;

    fn mul(self, rhs: Num) -> Self::Output {
        self * rhs.0 as i8
    }
}

impl Mul<Num> for i16 {
    type Output = i16;

    fn mul(self, rhs: Num) -> Self::Output {
        self * rhs.0 as i16
    }
}

impl Mul<Num> for i32 {
    type Output = i32;

    fn mul(self, rhs: Num) -> Self::Output {
        self * rhs.0 as i32
    }
}

impl Mul<Num> for i64 {
    type Output = i64;

    fn mul(self, rhs: Num) -> Self::Output {
        self * rhs.0 as i64
    }
}

impl Mul<Num> for isize {
    type Output = isize;

    fn mul(self, rhs: Num) -> Self::Output {
        self * rhs.0 as isize
    }
}

impl Sub<Num> for u8 {
    type Output = u8;

    fn sub(self, rhs: Num) -> Self::Output {
        self - rhs.0 as u8
    }
}

impl Sub<Num> for u16 {
    type Output = u16;

    fn sub(self, rhs: Num) -> Self::Output {
        self - rhs.0 as u16
    }
}

impl Sub<Num> for u32 {
    type Output = u32;

    fn sub(self, rhs: Num) -> Self::Output {
        self - rhs.0 as u32
    }
}

impl Sub<Num> for u64 {
    type Output = u64;

    fn sub(self, rhs: Num) -> Self::Output {
        self - rhs.0 as u64
    }
}

impl Sub<Num> for usize {
    type Output = usize;

    fn sub(self, rhs: Num) -> Self::Output {
        self - rhs.0
    }
}

impl Sub<Num> for i8 {
    type Output = i8;

    fn sub(self, rhs: Num) -> Self::Output {
        self - rhs.0 as i8
    }
}

impl Sub<Num> for i16 {
    type Output = i16;

    fn sub(self, rhs: Num) -> Self::Output {
        self - rhs.0 as i16
    }
}

impl Sub<Num> for i32 {
    type Output = i32;

    fn sub(self, rhs: Num) -> Self::Output {
        self - rhs.0 as i32
    }
}

impl Sub<Num> for i64 {
    type Output = i64;

    fn sub(self, rhs: Num) -> Self::Output {
        self - rhs.0 as i64
    }
}

impl Sub<Num> for isize {
    type Output = isize;

    fn sub(self, rhs: Num) -> Self::Output {
        self - rhs.0 as isize
    }
}

impl Div<Num> for u8 {
    type Output = u8;

    fn div(self, rhs: Num) -> Self::Output {
        self / rhs.0 as u8
    }
}

impl Div<Num> for u16 {
    type Output = u16;

    fn div(self, rhs: Num) -> Self::Output {
        self / rhs.0 as u16
    }
}

impl Div<Num> for u32 {
    type Output = u32;

    fn div(self, rhs: Num) -> Self::Output {
        self / rhs.0 as u32
    }
}

impl Div<Num> for u64 {
    type Output = u64;

    fn div(self, rhs: Num) -> Self::Output {
        self / rhs.0 as u64
    }
}

impl Div<Num> for usize {
    type Output = usize;

    fn div(self, rhs: Num) -> Self::Output {
        self / rhs.0
    }
}

impl Div<Num> for i8 {
    type Output = i8;

    fn div(self, rhs: Num) -> Self::Output {
        self / rhs.0 as i8
    }
}

impl Div<Num> for i16 {
    type Output = i16;

    fn div(self, rhs: Num) -> Self::Output {
        self / rhs.0 as i16
    }
}

impl Div<Num> for i32 {
    type Output = i32;

    fn div(self, rhs: Num) -> Self::Output {
        self / rhs.0 as i32
    }
}

impl Div<Num> for i64 {
    type Output = i64;

    fn div(self, rhs: Num) -> Self::Output {
        self / rhs.0 as i64
    }
}

impl Div<Num> for isize {
    type Output = isize;

    fn div(self, rhs: Num) -> Self::Output {
        self / rhs.0 as isize
    }
}
