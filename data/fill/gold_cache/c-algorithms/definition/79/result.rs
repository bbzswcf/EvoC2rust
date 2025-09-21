#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct _HashTableEntry {
    pub pair: HashTablePair,
    pub next: Ptr<HashTableEntry>,
}
