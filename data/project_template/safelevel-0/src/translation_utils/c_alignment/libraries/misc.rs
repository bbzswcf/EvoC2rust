use crate::translation_utils::*;

use std::path::Path;

pub fn basename(mut path: Ptr<u8>) -> Ptr<u8> {
    let length = c_strlen!(path);
    let mut index = length - 1;
    while index > 0 {
        if path[index] == b'/' {
            return path + index + 1;
        }
        index -= 1;
    }
    path
}

macro_rules! c_basename {
    ($path: expr) => {
        basename($path)
    };
}

pub(crate) use c_basename;

macro_rules! c__line__ {
    () => {
        line!()
    };
}
pub(crate) use c__line__;

macro_rules! __LINE__ {
    () => {
        c__line__!()
    };
}
pub(crate) use __LINE__;

macro_rules! c__file__ {
    () => {
        CStr::from(format!("{}\0", file!()).as_bytes())
    };
}
pub(crate) use c__file__;

macro_rules! __FILE__ {
    () => {
        c__file__!()
    };
}
pub(crate) use __FILE__;

macro_rules! c__function__ {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        let res = name.strip_suffix("::f").unwrap();
        let res = res.split("::").last().unwrap();
        CStr::from(format!("{}\0", res).as_bytes())
    }};
}

pub(crate) use c__function__;

macro_rules! __FUNCTION__ {
    () => {
        c__function__!()
    };
}
pub(crate) use __FUNCTION__;

macro_rules! __FUNC__ {
    () => {
        c__function__!()
    };
}
pub(crate) use __FUNC__;

