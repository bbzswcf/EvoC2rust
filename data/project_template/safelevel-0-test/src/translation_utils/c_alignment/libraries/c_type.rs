use crate::translation_utils::*;

pub fn isalnum<I: Integer>(c: I) -> i32 {
    if char::from(c.as_usize() as u8).is_alphanumeric() {
        1
    } else {
        0
    }
}

pub fn tolower<I: Integer>(c: I) -> i32 {
    char::from(c.as_usize() as u8).to_ascii_lowercase() as i32
}

pub fn toupper<I: Integer>(c: I) -> i32 {
    char::from(c.as_usize() as u8).to_ascii_uppercase() as i32
}

macro_rules! c_tolower {
    ($c: expr) => {
        tolower($c)
    };
}

pub(crate) use c_tolower;

macro_rules! c_toupper {
    ($c: expr) => {
        toupper($c)
    };
}

pub(crate) use c_toupper;
