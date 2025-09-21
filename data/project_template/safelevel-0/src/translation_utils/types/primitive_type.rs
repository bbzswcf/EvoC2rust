use crate::translation_utils::*;

pub trait PrimitiveType: Sized + Copy + PartialOrd + PartialEq {}

impl PrimitiveType for i8 {}
impl PrimitiveType for i16 {}
impl PrimitiveType for i32 {}
impl PrimitiveType for i64 {}
impl PrimitiveType for i128 {}
impl PrimitiveType for isize {}
impl PrimitiveType for u8 {}
impl PrimitiveType for u16 {}
impl PrimitiveType for u32 {}
impl PrimitiveType for u64 {}
impl PrimitiveType for u128 {}
impl PrimitiveType for usize {}
impl PrimitiveType for f32 {}
impl PrimitiveType for f64 {}
impl PrimitiveType for bool {}

pub type Void = u8;
pub type Char = u8;

pub type GenericPtr<T> = Ptr<T>;

pub type VoidPtr = GenericPtr<Void>;
