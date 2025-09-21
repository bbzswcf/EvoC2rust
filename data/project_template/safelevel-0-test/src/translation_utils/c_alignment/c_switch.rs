use crate::translation_utils::*;

macro_rules! c_switch {
    // Single case
    ($input:expr; $first:expr => $execute:expr$(,)?) => {
        let mut __executed = false;
        let mut __breaked = false;
        while !__breaked {
            __breaked = true;
            if __executed || $input == $first {  __executed = true; $execute; }
            __breaked = false;
            break;
        }
        while !__executed || !__breaked { break; }
    };

    // Single case with else
    ($input:expr; $first:expr => $execute:expr, _ => $execute_last:expr$(,)?) => {
        let mut __executed = false;
        let mut __breaked = false;
        while !__breaked {
            __breaked = true;
            if __executed || $input == $first { __executed = true; $execute;  }
            __breaked = false;
            break;
        }
        while !__executed || !__breaked { $execute_last; break; }
    };

    // Multi-case
    ($input:expr; $first:expr => $execute:expr, $($rest:expr => $exec:expr),+$(,)?) => {
        let mut __executed = false;
        let mut __breaked = false;
        while !__breaked {
            __breaked = true;
            if __executed || $input == $first { __executed = true; $execute;  }
            __breaked = false;
            break;
        }
        $(
            while !__breaked {
                __breaked = true;
                if __executed || $input == $rest {  __executed = true; $exec; }
                __breaked = false;
                break;
            }
        )*
        while !__executed || !__breaked { break; }
    };

    // Multi-case with else
    ($input:expr; $first:expr => $execute:expr, $($rest:expr => $exec:expr),+, _ => $execute_last:expr$(,)?) => {
        let mut __executed = false;
        let mut __breaked = false;
        while !__breaked {
            __breaked = true;
            if __executed || $input == $first { __executed = true; $execute; }
            __breaked = false;
            break;
        }
        $(
            while !__breaked {
                __breaked = true;
                if __executed || $input == $rest { __executed = true; $exec; }
                __breaked = false;
                break;
            }
        )*        
        while !__executed || !__breaked { $execute_last; break; }
    };
}

pub(crate) use c_switch;

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! MY_1 { () => { 1 } }
    #[test]
    fn test_single_case() {
        let input = 2;
        let mut result = 0;
        c_switch!(input;
            MY_1!() => { result = 1; break; },
            2 => { result = 2; },
            3 => { result = 3; },
            _ => { result = 4; break; }
        );
        assert_eq!(result, 4);
    }

    #[test]
    fn test_multiple_case() {
        let input = 3;
        let mut result = 0;
        c_switch!(input;
            MY_1!() => { result |= 1; break; },
            2 => { result |= 2; },
            3 => { result |= 4; break; },
            4 => { result |= 8; },
            5 => { result |= 32; },
            _ => { result |= 16; break; }
        );

        assert_eq!(result, 4);
    }
}