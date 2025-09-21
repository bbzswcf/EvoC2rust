use crate::translation_utils::*;

// Copy From dyn-fmt lib

use core::fmt::{self, Display};
use core::hint::unreachable_unchecked;

pub trait AsStrFormatExt: AsRef<str> {
    fn format<'a, T: Display + ?Sized + 'a>(
        &self,
        args: impl IntoIterator<Item = &'a T> + Clone,
    ) -> String {
        format!("{}", Arguments::new(self, args))
    }
}

impl<T: AsRef<str>> AsStrFormatExt for T {}

#[macro_export]
macro_rules! dyn_write {
    ($dst:expr, $fmt:expr, $args:expr $(,)?) => {
        $crate::core::write!($dst, "{}", $crate::Arguments::new($fmt, $args))
    };
}

#[derive(Clone, Debug)]
pub struct Arguments<
    'a,
    F: AsRef<str>,
    T: Display + ?Sized + 'a,
    I: IntoIterator<Item = &'a T> + Clone,
> {
    fmt: F,
    args: I,
}

impl<'a, F: AsRef<str>, T: Display + ?Sized + 'a, I: IntoIterator<Item = &'a T> + Clone>
    Arguments<'a, F, T, I>
{
    pub fn new(fmt: F, args: I) -> Self {
        Arguments { fmt, args }
    }
}

impl<'a, F: AsRef<str>, T: Display + ?Sized + 'a, I: IntoIterator<Item = &'a T> + Clone> Display
    for Arguments<'a, F, T, I>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        #[derive(Eq, PartialEq)]
        enum Brace {
            Left,
            Right,
        }
        fn as_brace(c: u8) -> Option<Brace> {
            match c {
                b'{' => Some(Brace::Left),
                b'}' => Some(Brace::Right),
                _ => None,
            }
        }
        let mut args = self.args.clone().into_iter();
        let mut fmt = self.fmt.as_ref();
        let mut piece_end = 0;
        enum State {
            Piece,
            Arg,
        }
        let mut state = State::Piece;
        loop {
            match state {
                State::Piece => match fmt.as_bytes()[piece_end..].first() {
                    None => {
                        fmt.fmt(f)?;
                        break;
                    }
                    Some(&b) => match as_brace(b) {
                        Some(b) => {
                            fmt[..piece_end].fmt(f)?;
                            fmt = &fmt[(piece_end + 1)..];
                            if fmt.is_empty() {
                                break;
                            }
                            match b {
                                Brace::Left => {
                                    piece_end = 0;
                                    state = State::Arg;
                                }
                                Brace::Right => {
                                    piece_end = 1;
                                    state = State::Piece;
                                }
                            };
                        }
                        None => {
                            piece_end += 1;
                        }
                    },
                },
                State::Arg => match fmt.as_bytes().first() {
                    None => unsafe { unreachable_unchecked() },
                    Some(&b'}') => {
                        if let Some(arg) = args.next() {
                            arg.fmt(f)?;
                        }
                        fmt = &fmt[1..];
                        state = State::Piece;
                    }
                    Some(_) => {
                        piece_end = 1;
                        state = State::Piece;
                    }
                },
            }
        }
        Ok(())
    }
}

// additional

pub type VaList<'a> = &'a [&'a dyn Display];

macro_rules! va_format {
    ($fmt:expr, $args:expr) => {
        $fmt.to_string().format($args)
    };
}

pub(crate) use va_format;

macro_rules! c_va_start {
    ($args:expr, $last:expr) => {};
}

pub(crate) use c_va_start;

macro_rules! c_va_end {
    ($args:expr) => {};
}

pub(crate) use c_va_end;

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_macro {
        ($fmt:expr) => {
            test_func($fmt, &[])
        };
        ($fmt:expr, $($args:expr), *) => {
            test_func($fmt, &[$(&$args), *])
        };
    }

    fn test_func(fmt: &str, args: VaList) -> String {
        va_format!(fmt, args)
    }

    #[test]
    fn test_va_format() {
        assert_eq!(
            va_format!(cstr!("Hello, {}"), &[&"world!"]),
            "Hello, world!"
        );
    }

    #[test]
    fn test_va_format_2() {
        assert_eq!(test_func("Hello, {}", &[&"world!"]), "Hello, world!");
    }

    #[test]
    fn test_va_format_3() {
        assert_eq!(test_macro!("Hello, {}", "world!"), "Hello, world!");
    }

    #[test]
    fn test_va_format_4() {
        assert_eq!(test_macro!("Hello, world!"), "Hello, world!");
    }

    #[test]
    fn test_va_format_5() {
        assert_eq!(
            test_macro!("Hello, {} {} {} {} {}", "world!", 1, 2, 3, 3.14),
            "Hello, world! 1 2 3 3.14"
        );
    }
}
