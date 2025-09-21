#[repr(C)]
#[derive(Default)]
pub struct _TrieNode {
    pub data: TrieValue,
    pub use_count: u32,
    pub next: Array<Ptr<TrieNode>, 256>,
}
