use crate::translation_utils::*;

use std::sync::{LazyLock, Mutex, MutexGuard};

pub struct Global<T>(pub LazyLock<Mutex<T>>);

impl<T> Global<T> {
    pub const fn new(value_function: fn() -> Mutex<T>) -> Self {
        let mut result: LazyLock<Mutex<T>> = LazyLock::new(value_function);
        Global(result)
    }

    pub fn lock(&self) -> MutexGuard<T> {
        self.0.lock().unwrap()
    }
}

macro_rules! global {
    ($value:expr) => {
        Global::new(|| std::sync::Mutex::new($value))
    };
}

pub(crate) use global;

pub type MyFunction = FuncPtr<fn(i32, i32) -> i32>;

static GLOBAL: Global<MyFunction> = global!(NULL!());

#[test]
fn test_global() {
    assert!((*GLOBAL.lock()).0.is_none());
    // GLOBAL.get()[0] = 1;
    // assert_eq!(GLOBAL.get()[0], 1);
}
