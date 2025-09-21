use crate::translation_utils::*;

use core::cmp::Ordering;

pub fn strcmp(mut str1: Ptr<Char>, mut str2: Ptr<Char>) -> i32 {
    let mut p1 = str1;
    let mut p2 = str2;
    while *p1 != 0 && *p2 != 0 {
        if *p1 < *p2 {
            return -1;
        } else if *p1 > *p2 {
            return 1;
        }
        p1 += 1;
        p2 += 1;
    }
    if *p1 == 0 && *p2 == 0 {
        return 0;
    } else if *p1 == 0 {
        return -1;
    } else {
        return 1;
    }
}

pub fn strncmp(mut str1: Ptr<Char>, mut str2: Ptr<Char>, n: usize) -> i32 {
    let mut p1 = str1;
    let mut p2 = str2;
    let mut i = 0;
    while i < n && *p1 != 0 && *p2 != 0 {
        if *p1 < *p2 {
            return -1;
        } else if *p1 > *p2 {
            return 1;
        }
        p1 += 1;
        p2 += 1;
        i += 1;
    }
    if i == n {
        return 0;
    } else if *p1 == 0 && *p2 == 0 {
        return 0;
    } else if *p1 == 0 {
        return -1;
    } else {
        return 1;
    }
}

pub fn strlen(mut str1: Ptr<Char>) -> usize {
    let mut length = 0;
    while str1[length] != 0 {
        length += 1;
    }
    length
}

pub fn strdup(mut str1: Ptr<Char>) -> Ptr<Char> {
    let mut length = strlen(str1) + 1;
    let mut new_str1: Ptr<Char> = c_malloc!(length);
    for i in 0..length {
        new_str1[i] = str1[i];
    }
    new_str1
}

macro_rules! c_strcmp {
    ($s1: expr, $s2: expr) => {
        strcmp($s1.cast(), $s2.cast())
    };
}

pub(crate) use c_strcmp;

macro_rules! c_strncmp {
    ($s1: expr, $s2: expr, $n: expr) => {
        strncmp($s1.cast(), $s2.cast(), $n)
    };
}

pub(crate) use c_strncmp;

macro_rules! c_strlen {
    ($s: expr) => {
        strlen($s.cast())
    };
}

pub(crate) use c_strlen;

macro_rules! c_strdup {
    ($s: expr) => {
        strdup($s.cast())
    };
}

pub(crate) use c_strdup;

pub fn printf(format: Ptr<u8>, va: VaList) -> i32 {
    let mut fmt = format.to_string();
    let mut tmp = va_format!(fmt, va);
    print!("{}", tmp);
    return tmp.as_bytes().len() as i32;
}

macro_rules! c_printf {
    ($fmt: expr) => {
        printf($fmt.cast(), &[])
    };
    ($fmt: expr, $($arg: expr), *) => {
        printf($fmt.cast(), &[$(&$arg), *])
    };
}

pub(crate) use c_printf;

pub fn sprintf(mut buf: Ptr<u8>, mut format: Ptr<u8>, va: VaList) -> i32 {
    let mut fmt = format.to_string();
    let mut tmp = va_format!(fmt, va);
    let mut length = tmp.as_bytes().len() as i32;
    for i in 0..length {
        buf[i] = tmp.as_bytes()[i as usize];
    }
    buf[length] = 0;
    return length;
}

macro_rules! c_sprintf {
    ($buf: expr, $fmt: expr) => {
        sprintf($buf.cast(), $fmt, &[])
    };
    ($buf: expr, $fmt: expr, $($arg: expr), *) => {
        sprintf($buf.cast(), $fmt, &[$(&$arg), *])
    };
}

pub(crate) use c_sprintf;

pub fn sprintf_s(mut buf: Ptr<u8>, size: usize, mut format: Ptr<u8>, va: VaList) -> i32 {
    if buf == null!() || size == 0 || format == null!() {
        return -1;
    }
    let mut fmt = format.to_string();
    let mut tmp = va_format!(fmt, va);
    let mut length = tmp.as_bytes().len() as i32;
    for i in 0..length {
        buf[i] = tmp.as_bytes()[i as usize];
    }
    buf[length] = 0;
    return length;
}

macro_rules! c_sprintf_s {
    ($buf: expr, $size: expr, $fmt: expr) => {
        sprintf_s($buf.cast(), $size.cast(), $fmt, &[])
    };
    ($buf: expr, $size: expr, $fmt: expr, $($arg: expr), *) => {
        sprintf_s($buf.cast(), $size.cast(), $fmt, &[$(&$arg), *])
    };
}

pub(crate) use c_sprintf_s;

pub fn snprintf(mut buf: Ptr<u8>, size: usize, format: Ptr<u8>, va: VaList) -> i32 {
    let mut fmt = format.to_string();
    let mut tmp = va_format!(fmt, va);
    let mut length = tmp.as_bytes().len() as i32;
    if length > (size - 1) as i32 {
        length = (size - 1) as i32;
    }
    for i in 0..length {
        buf[i] = tmp.as_bytes()[i as usize];
    }
    buf[length as usize] = 0;
    return length;
}

macro_rules! c_snprintf {
    ($buf: expr, $size: expr, $fmt: expr) => {
        snprintf($buf.cast(), $size.cast(), $fmt, &[])
    };
    ($buf: expr, $size: expr, $fmt: expr, $($arg: expr), *) => {
        snprintf($buf.cast(), $size.cast(), $fmt, &[$(&$arg), *])
    };
}

pub(crate) use c_snprintf;

pub fn snprintf_s(
    mut buf: Ptr<u8>,
    size: usize,
    mut count: usize,
    format: Ptr<u8>,
    va: VaList,
) -> i32 {
    if buf == null!() || size == 0 || format == null!() {
        return -1;
    }
    let mut fmt = format.to_string();
    let mut tmp = va_format!(fmt, va);
    let mut length = tmp.as_bytes().len() as i32;
    if count > (size - 1) {
        count = size - 1;
    }
    if length > count as i32 {
        length = count as i32;
    }
    for i in 0..length {
        buf[i] = tmp.as_bytes()[i as usize];
    }
    buf[length as usize] = 0;
    return length;
}

macro_rules! c_snprintf_s {
    ($buf: expr, $size: expr, $count: expr, $fmt: expr) => {
        snprintf_s($buf.cast(), $size.cast(), $count.cast(), $fmt, &[])
    };
    ($buf: expr, $size: expr, $count: expr, $fmt: expr, $($arg: expr), *) => {
        snprintf_s($buf.cast(), $size.cast(), $count.cast(), $fmt, &[$(&$arg), *])
    };
}

pub(crate) use c_snprintf_s;

macro_rules! c_vsnprintf_s {
    ($buf: expr, $size: expr, $count: expr, $fmt: expr, $va: expr) => {
        snprintf_s($buf.cast(), $size.cast(), $count.cast(), $fmt, $va)
    };
}

pub(crate) use c_vsnprintf_s;

pub fn strrchr(mut str1: Ptr<u8>, mut ch: u8) -> Ptr<u8> {
    let mut length = strlen(str1);
    let mut index = length as i32 - 1;
    while index >= 0 {
        if str1[index] == ch {
            return str1 + index;
        }
        index -= 1;
    }
    null!()
}

macro_rules! c_strrchr {
    ($s: expr, $c: expr) => {
        strrchr($s.cast(), $c)
    };
}

pub(crate) use c_strrchr;

pub fn strcpy(mut dest: Ptr<u8>, mut src: Ptr<u8>) -> Ptr<u8> {
    let mut length = strlen(src);
    for i in 0..length {
        dest[i] = src[i];
    }
    dest[length] = 0;
    dest
}

macro_rules! c_strcpy {
    ($dest: expr, $src: expr) => {
        strcpy($dest.cast(), $src.cast())
    };
}

pub(crate) use c_strcpy;

pub fn strcpy_s(
    mut dest: Ptr<u8>,
    mut size: usize,
    mut src: Ptr<u8>,
) -> i32 {
    if dest == null!() || size == 0 || src == null!() {
        return -1;
    }
    let mut length = strlen(src);
    if length > (size - 1) as usize {
        length = (size - 1) as usize;
    }
    for i in 0..length {
        dest[i] = src[i];
    }
    dest[length as usize] = 0;
    return length as i32;
}

macro_rules! c_strcpy_s {
    ($dest: expr, $size: expr, $src: expr) => {
        strcpy_s($dest.cast(), $size.cast(), $src.cast())
    };
}

pub(crate) use c_strcpy_s;