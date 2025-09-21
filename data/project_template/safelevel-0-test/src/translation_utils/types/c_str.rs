use crate::translation_utils::*;

use core::ops::*;

use core::ptr::NonNull;

pub type CStr = Ptr<Char>;

impl CStr {
    pub const fn from(data: &[Char]) -> Self {
        let length = data.len();
        let ptr: NonNull<Char> = unsafe { NonNull::new_unchecked(data.as_ptr() as *mut Char) };
        Ptr(Some(ptr))
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        let mut index = 0;
        let mut c: CStr = *self;
        while index < c_strlen!(c) {
            result.push(self[index] as char);
            index += 1;
        }
        result
    }
}

impl core::fmt::Display for CStr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

macro_rules! cstr {
    ($string: literal) => {
        CStr::from(concat!($string, "\0").as_bytes())
    };
}

pub(crate) use cstr;

macro_rules! bstr {
    ($bstring: expr) => {
        CStr::from($bstring)
    };
}

pub(crate) use bstr;