pub fn binomial_tree_ref(mut tree: Ptr<BinomialTree>) {
    if (tree != NULL!()).as_bool() {
        tree.refcount.prefix_plus_plus();
    }
}
