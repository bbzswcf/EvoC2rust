#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct _HashTablePair {
    pub key: HashTableKey,
    pub value: HashTableValue,
}

pub type HashTablePair = _HashTablePair;
