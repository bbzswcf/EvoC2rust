pub fn trie_find_end(mut trie: Ptr<Trie>, mut key: Ptr<u8>) -> Ptr<TrieNode> {
    let mut node: Ptr<TrieNode> = Default::default();
    let mut p: Ptr<u8> = Default::default();

    node = trie.root_node.cast();

    c_for!(p = key; *p != 0; p.prefix_plus_plus(); {
        if (node == NULL!()).as_bool() {
            return NULL!();
        }

        node = node.next[(*p).cast::<u8>()].cast();
    });

    return node.cast();
}
