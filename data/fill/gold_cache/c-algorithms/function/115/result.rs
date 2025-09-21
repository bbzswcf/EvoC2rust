pub fn trie_new() -> Ptr<Trie> {
    let mut new_trie: Ptr<Trie> = c_malloc!(c_sizeof!(Trie));

    if (new_trie == NULL!()).as_bool() {
        return NULL!();
    }

    new_trie.root_node = NULL!();

    return new_trie.cast();
}
