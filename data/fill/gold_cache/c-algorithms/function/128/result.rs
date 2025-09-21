pub fn trie_num_entries(mut trie: Ptr<Trie>) -> u32 {
    if (trie.root_node == NULL!()).as_bool() {
        return 0;
    } else {
        return trie.root_node.use_count.cast();
    }
}
