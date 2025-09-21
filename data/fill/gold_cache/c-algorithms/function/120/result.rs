pub fn trie_find_end_binary(mut trie: Ptr<Trie>, mut key: Ptr<u8>, mut key_length: i32) -> Ptr<TrieNode> {
    let mut node: Ptr<TrieNode> = Default::default();
    let mut j: i32 = Default::default();
    let mut c: i32 = Default::default();

    node = trie.root_node.cast();

    c_for!(j = 0; j < key_length; j.suffix_plus_plus(); {
        if (node == NULL!()).as_bool() {
            return NULL!();
        }

        c = key[j].cast::<u8>().cast::<i32>();

        node = node.next[c].cast();
    });

    return node.cast();
}
