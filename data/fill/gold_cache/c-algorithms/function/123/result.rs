pub fn trie_insert_binary(mut trie: Ptr<Trie>, mut key: Ptr<u8>, mut key_length: i32, mut value: TrieValue) -> i32 {
    let mut rover: Ptr<Ptr<TrieNode>> = Default::default();
    let mut node: Ptr<TrieNode> = Default::default();
    let mut p: i32 = Default::default();
    let mut c: i32 = Default::default();

    if (value == TRIE_NULL!()) {
        return 0;
    }

    node = trie_find_end_binary(trie, key, key_length);

    if (node != NULL!()) && (node.data != TRIE_NULL!()) {
        node.data = value;
        return 1;
    }

    rover = c_ref!(trie.root_node);
    p = 0;

    loop {
        node = *rover;

        if (node == NULL!()) {
            node = c_calloc!(1, c_sizeof!(TrieNode));

            if (node == NULL!()) {
                trie_insert_rollback(trie, key);
                return 0;
            }

            node.data = TRIE_NULL!();
            *rover = node;
        }

        node.use_count.suffix_plus_plus();

        c = key[p].cast::<u8>().cast::<i32>();

        if (p == key_length) {
            node.data = value;
            break;
        }

        rover = c_ref!(node.next[c]);
        p.suffix_plus_plus();
    }

    return 1;
}
