use crate::translation_utils::*;

use core::any::Any;
use core::ops::*;
use core::ptr::NonNull;

use std::collections::BTreeMap;
use std::sync::RwLock;

pub struct ResourceHeap {
    heap_mutex: RwLock<()>,
    heap: BTreeMap<usize, Vec<u8>>,
}

pub static mut RESOURCE_HEAP: ResourceHeap = ResourceHeap::new();

impl ResourceHeap {
    pub const fn new() -> Self {
        Self {
            heap_mutex: RwLock::new(()),
            heap: BTreeMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        if let Ok(_) = self.heap_mutex.read() {
            self.heap.len()
        } else {
            panic!("ResourceHeap is poisoned!");
        }
    }

    pub fn alloc(&mut self, count: usize) -> Ptr<Void> {
        if let Ok(_) = self.heap_mutex.write() {
            let mut arr: Vec<u8> = vec![0; count];
            let nonnull = NonNull::new(arr.as_mut_ptr() as *mut u8).unwrap();
            self.heap.insert(nonnull.as_ptr() as usize, arr);
            let result = Ptr(Some(nonnull));
            result
        } else {
            panic!("ResourceHeap is poisoned!");
        }
    }

    pub fn realloc(&mut self, ptr: Ptr<Void>, count: usize) -> Ptr<Void> {
        if let Ok(_) = self.heap_mutex.write() {
            if let Some(nonnull) = ptr.0 {
                let key = nonnull.as_ptr() as usize;
                let mut remove_value = self.heap.remove(&(nonnull.as_ptr() as usize));
                if remove_value.is_none() {
                    panic!("ResourceHeap realloc failed!");
                }
                let mut remove_value = remove_value.unwrap();
                remove_value.resize(count, 0);
                let nonnull = NonNull::new(remove_value.as_mut_ptr() as *mut u8).unwrap();
                self.heap.insert(nonnull.as_ptr() as usize, remove_value);
                let result = Ptr(Some(nonnull));
                result
            } else {
                let mut arr: Vec<u8> = vec![0; count];
                let nonnull = NonNull::new(arr.as_mut_ptr() as *mut u8).unwrap();
                self.heap.insert(nonnull.as_ptr() as usize, arr);
                let result = Ptr(Some(nonnull));
                result
            }
        } else {
            panic!("ResourceHeap is poisoned!");
        }
    }

    pub fn dealloc(&mut self, ptr: Ptr<Void>) {
        if let Ok(_) = self.heap_mutex.write() {
            if let Some(nonnull) = ptr.0 {
                let key = nonnull.as_ptr() as usize;
                let remove_value = self.heap.remove(&key);
                if remove_value.is_none() {
                    panic!("ResourceHeap dealloc failed!");
                }
            }
        } else {
            panic!("ResourceHeap is poisoned!");
        }
    }
}

macro_rules! test_no_memory_leak {
    () => {
        assert_eq!(unsafe { RESOURCE_HEAP.len() }, 0);
    };
}

pub(crate) use test_no_memory_leak;
