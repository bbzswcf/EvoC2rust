use crate::translation_utils::*;

pub fn atoi(mut str1: Ptr<Char>) -> i64 {
    let mut result = 0;
    let mut sign = 1;
    let mut i = 0;
    let length = c_strlen!(str1);
    if str1[0] == b'-' {
        sign = -1;
        i = 1;
    }
    while i < length {
        let curr_char: u8 = str1[i];
        if curr_char == b'\0' {
            break;
        }
        if curr_char < b'0' || curr_char > b'9' {
            return 0;
        }
        result = result * 10 + (curr_char - b'0') as i64;
        i += 1;
    }
    sign * result
}

macro_rules! c_atoi {
    ($s: expr) => {
        atoi($s.cast())
    };
}

pub(crate) use c_atoi;
