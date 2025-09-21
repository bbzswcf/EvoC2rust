#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct _HashTable {
    pub table: Ptr<Ptr<HashTableEntry>>,
    pub table_size: u32,
    pub hash_func: HashTableHashFunc,
    pub equal_func: HashTableEqualFunc,
    pub key_free_func: HashTableKeyFreeFunc,
    pub value_free_func: HashTableValueFreeFunc,
    pub entries: u32,
    pub prime_index: u32,
}
