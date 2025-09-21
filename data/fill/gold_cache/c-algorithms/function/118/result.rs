pub fn trie_free(mut trie: Ptr<Trie>) {
    let mut free_list: Ptr<TrieNode> = NULL!();
    let mut node: Ptr<TrieNode> = Default::default();
    let mut i: i32 = Default::default();

    free_list = NULL!();

    if (trie.root_node != NULL!()).as_bool() {
        trie_free_list_push(c_ref!(free_list).cast(), trie.root_node.cast());
    }

    while (free_list != NULL!()).as_bool() {
        node = trie_free_list_pop(c_ref!(free_list).cast());

        c_for!(let mut i: i32 = 0; i < 256; i.prefix_plus_plus(); {
            if (node.next[i] != NULL!()).as_bool() {
                trie_free_list_push(c_ref!(free_list).cast(), node.next[i].cast());
            }
        });

        c_free!(node);
    }

    c_free!(trie);
}
