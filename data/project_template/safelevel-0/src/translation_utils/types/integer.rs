use crate::translation_utils::*;

pub trait Integer: PrimitiveType {
    fn as_bool(self) -> bool;
    fn as_usize(self) -> usize;
    fn from_usize(value: usize) -> Self;
}

impl Integer for i8 {
    fn as_bool(self) -> bool {
        self != 0
    }

    fn as_usize(self) -> usize {
        self as usize
    }

    fn from_usize(value: usize) -> Self {
        value as i8
    }
}

impl Integer for i16 {
    fn as_bool(self) -> bool {
        self != 0
    }

    fn as_usize(self) -> usize {
        self as usize
    }

    fn from_usize(value: usize) -> Self {
        value as i16
    }
}

impl Integer for i32 {
    fn as_bool(self) -> bool {
        self != 0
    }

    fn as_usize(self) -> usize {
        self as usize
    }

    fn from_usize(value: usize) -> Self {
        value as i32
    }
}

impl Integer for i64 {
    fn as_bool(self) -> bool {
        self != 0
    }

    fn as_usize(self) -> usize {
        self as usize
    }

    fn from_usize(value: usize) -> Self {
        value as i64
    }
}

impl Integer for i128 {
    fn as_bool(self) -> bool {
        self != 0
    }

    fn as_usize(self) -> usize {
        self as usize
    }

    fn from_usize(value: usize) -> Self {
        value as i128
    }
}

impl Integer for isize {
    fn as_bool(self) -> bool {
        self != 0
    }

    fn as_usize(self) -> usize {
        self as usize
    }

    fn from_usize(value: usize) -> Self {
        value as isize
    }
}

impl Integer for u8 {
    fn as_bool(self) -> bool {
        self != 0
    }

    fn as_usize(self) -> usize {
        self as usize
    }

    fn from_usize(value: usize) -> Self {
        value as u8
    }
}

impl Integer for u16 {
    fn as_bool(self) -> bool {
        self != 0
    }

    fn as_usize(self) -> usize {
        self as usize
    }

    fn from_usize(value: usize) -> Self {
        value as u16
    }
}

impl Integer for u32 {
    fn as_bool(self) -> bool {
        self != 0
    }

    fn as_usize(self) -> usize {
        self as usize
    }

    fn from_usize(value: usize) -> Self {
        value as u32
    }
}

impl Integer for u64 {
    fn as_bool(self) -> bool {
        self != 0
    }

    fn as_usize(self) -> usize {
        self as usize
    }

    fn from_usize(value: usize) -> Self {
        value as u64
    }
}

impl Integer for u128 {
    fn as_bool(self) -> bool {
        self != 0
    }

    fn as_usize(self) -> usize {
        self as usize
    }

    fn from_usize(value: usize) -> Self {
        value as u128
    }
}

impl Integer for usize {
    fn as_bool(self) -> bool {
        self != 0
    }

    fn as_usize(self) -> usize {
        self
    }

    fn from_usize(value: usize) -> Self {
        value
    }
}

impl Integer for bool {
    fn as_bool(self) -> bool {
        self
    }

    fn as_usize(self) -> usize {
        if self {
            1
        } else {
            0
        }
    }

    fn from_usize(value: usize) -> Self {
        value != 0
    }
}

impl<I1: Integer, I2: Integer> CastFrom<I1> for I2 {
    fn cast_from(value: &mut I1) -> I2 {
        I2::from_usize(value.as_usize())
    }
}