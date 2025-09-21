use crate::translation_utils::*;

macro_rules! c_for {
    (; $($rest: tt)*) => {
        c_for!((); $($rest)*)
    };
    ($($init: stmt),+; ; $($rest: tt)*) => {
        c_for!($($init),+; !false; $($rest)*)
    };
    ($($init: stmt),+; $cond: expr; ; $body: block) => {
        c_for!{$($init),+; $cond; (); $body}
    };
    ($($init: stmt),+; $cond: expr; $($step: expr),+; $body: block) => {
        {
            $($init)+
            while $cond {
                let mut _first = true;
                let mut _continue = false;
                loop {
                    if !_first { _continue = true; break }
                    _first = false;

                    $body
                }
                if !_continue {
                    break
                }
                $($step;)+
            }
        }
    };
}

pub(crate) use c_for;