#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct _HashTableIterator {
    pub hash_table: Ptr<HashTable>,
    pub next_entry: Ptr<HashTableEntry>,
    pub next_chain: u32,
}
