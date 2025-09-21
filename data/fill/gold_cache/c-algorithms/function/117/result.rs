pub fn trie_free_list_pop(mut list: Ptr<Ptr<TrieNode>>) -> Ptr<TrieNode> {
    let mut result: Ptr<TrieNode> = Default::default();

    result = (*list).cast();
    *list = result.data.cast();

    return result.cast();
}
