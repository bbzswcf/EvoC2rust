use crate::translation_utils::*;

pub trait CFree {
    fn c_free(&mut self);
}

pub fn free<T: CFree>(mut ptr: T) {
    ptr.c_free();
}

impl<T: Default> CFree for Ptr<T> {
    fn c_free(&mut self) {
        unsafe {
            RESOURCE_HEAP.dealloc(self.cast());
        }
    }
}

macro_rules! c_free {
    () => {
        func!(free);
    };
    ($ptr:expr) => {
        $ptr.c_free();
    };
}
pub(crate) use c_free;
