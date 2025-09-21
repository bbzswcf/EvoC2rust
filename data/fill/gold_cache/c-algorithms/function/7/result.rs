pub fn rb_tree_free(mut tree: Ptr<RBTree>) {
    rb_tree_free_subtree(tree.root_node.cast());
    c_free!(tree);
}
