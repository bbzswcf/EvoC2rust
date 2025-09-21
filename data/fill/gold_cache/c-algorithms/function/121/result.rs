pub fn trie_insert_rollback(mut trie: Ptr<Trie>, mut key: Ptr<u8>) {
    let mut node: Ptr<TrieNode> = Default::default();
    let mut prev_ptr: Ptr<Ptr<TrieNode>> = Default::default();
    let mut next_node: Ptr<TrieNode> = Default::default();
    let mut next_prev_ptr: Ptr<Ptr<TrieNode>> = Default::default();
    let mut p: Ptr<u8> = Default::default();

    node = trie.root_node.cast();
    prev_ptr = c_ref!(trie.root_node).cast();
    p = key.cast();

    while (node != NULL!()).as_bool() {
        next_prev_ptr = c_ref!(node.next[*p]).cast();
        next_node = *next_prev_ptr;
        p += 1;

        node.use_count -= 1;

        if (node.use_count == 0).as_bool() {
            c_free!(node);

            if (prev_ptr != NULL!()).as_bool() {
                *prev_ptr = NULL!();
            }

            next_prev_ptr = NULL!();
        }

        node = next_node.cast();
        prev_ptr = next_prev_ptr.cast();
    }
}
