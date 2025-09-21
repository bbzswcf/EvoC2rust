pub fn trie_lookup_binary(mut trie: Ptr<Trie>, mut key: Ptr<u8>, mut key_length: i32) -> TrieValue {
    let mut node: Ptr<TrieNode> = Default::default();

    node = trie_find_end_binary(trie.cast(), key.cast(), key_length.cast());

    if (node != NULL!()).as_bool() {
        return node.data.cast();
    } else {
        return TRIE_NULL!();
    }
}
