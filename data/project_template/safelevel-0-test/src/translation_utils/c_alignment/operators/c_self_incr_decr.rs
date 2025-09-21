use crate::translation_utils::*;

pub trait CSelfIncrDecr {
    fn plus_plus(&mut self) -> Self;
    fn minus_minus(&mut self) -> Self;
    fn prefix_plus_plus(&mut self) -> Self;
    fn prefix_minus_minus(&mut self) -> Self;
    fn suffix_plus_plus(&mut self) -> Self;
    fn suffix_minus_minus(&mut self) -> Self;
}

impl <T> CSelfIncrDecr for Ptr<T> {
    fn plus_plus(&mut self) -> Self {
        let res = *self;
        *self += 1;
        res
    }

    fn minus_minus(&mut self) -> Self {
        let res = *self;
        *self -= 1;
        res
    }

    fn prefix_plus_plus(&mut self) -> Self {
        *self += 1;
        *self
    }

    fn prefix_minus_minus(&mut self) -> Self {
        *self -= 1;
        *self
    }

    fn suffix_plus_plus(&mut self) -> Self {
        let res = *self;
        *self += 1;
        res
    }

    fn suffix_minus_minus(&mut self) -> Self {
        let res = *self;
        *self -= 1;
        res
    }
}

macro_rules! c_self_incr_decr {
    ($Type:ty) => {
        impl CSelfIncrDecr for $Type {
            fn plus_plus(&mut self) -> Self {
                let res = *self;
                *self += 1;
                res
            }

            fn minus_minus(&mut self) -> Self {
                let res = *self;
                *self -= 1;
                res
            }

            fn prefix_plus_plus(&mut self) -> Self {
                *self += 1;
                *self
            }

            fn prefix_minus_minus(&mut self) -> Self {
                *self -= 1;
                *self
            }

            fn suffix_plus_plus(&mut self) -> Self {
                let res = *self;
                *self += 1;
                res
            }

            fn suffix_minus_minus(&mut self) -> Self {
                let res = *self;
                *self -= 1;
                res
            }
        }
    };
}

c_self_incr_decr!(i8);
c_self_incr_decr!(i16);
c_self_incr_decr!(i32);
c_self_incr_decr!(i64);
c_self_incr_decr!(i128);
c_self_incr_decr!(isize);
c_self_incr_decr!(u8);
c_self_incr_decr!(u16);
c_self_incr_decr!(u32);
c_self_incr_decr!(u64);
c_self_incr_decr!(u128);
c_self_incr_decr!(usize);

macro_rules! plus_plus {
    ($val:expr) => {
        $val.prefix_plus_plus()
    };
}

pub(crate) use plus_plus;

macro_rules! minus_minus {
    ($val:expr) => {
        $val.prefix_minus_minus()
    };
}

pub(crate) use minus_minus;
