pub fn trie_free_list_push(mut list: Ptr<Ptr<TrieNode>>, mut node: Ptr<TrieNode>) {
    node.data = (*list).cast();
    *list = node.cast();
}
