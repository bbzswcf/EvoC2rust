#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct _Trie {
    pub root_node: Ptr<TrieNode>,
}
