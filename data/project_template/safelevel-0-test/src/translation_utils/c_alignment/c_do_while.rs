use crate::translation_utils::*;

macro_rules! c_do {
    ($body:block while $cond:expr) => (
        {
        let mut __first = true;
        while core::mem::replace(&mut __first, false) || $cond
            $body
        }
    )
}

pub(crate) use c_do;

macro_rules! c_while {
    ($cond:expr; $body:block) => (
        while $cond
            $body
    )
}

pub(crate) use c_while;