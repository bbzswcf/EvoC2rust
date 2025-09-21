use crate::translation_utils::*;

pub trait CastInto<T> {
    fn cast_into(&mut self) -> T;
}

pub trait CastFrom<T> {
    fn cast_from(t: &mut T) -> Self;
}

impl<T1, T2> CastInto<T2> for T1
where
    T2: CastFrom<T1>,
{
    fn cast_into(&mut self) -> T2 {
        T2::cast_from(self)
    }
}

pub trait CastIntoTyped {
    fn cast<T>(&mut self) -> T
    where
        Self: CastInto<T>,
    {
        self.cast_into()
    }
}
impl<T> CastIntoTyped for T {}
