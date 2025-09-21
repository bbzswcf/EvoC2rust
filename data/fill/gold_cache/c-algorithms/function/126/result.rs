pub fn trie_lookup(mut trie: Ptr<Trie>, mut key: Ptr<u8>) -> TrieValue {
    let mut node: Ptr<TrieNode> = Default::default();

    node = trie_find_end(trie.cast(), key.cast()).cast();

    if (node != NULL!()).as_bool() {
        return node.data.cast();
    } else {
        return TRIE_NULL!();
    }
}
