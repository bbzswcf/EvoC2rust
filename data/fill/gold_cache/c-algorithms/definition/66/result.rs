#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct _SetIterator {
    pub set: Ptr<Set>,
    pub next_entry: Ptr<SetEntry>,
    pub next_chain: u32,
}
