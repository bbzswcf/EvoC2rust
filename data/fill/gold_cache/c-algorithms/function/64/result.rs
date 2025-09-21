pub fn avl_tree_free(mut tree: Ptr<AVLTree>) {
    avl_tree_free_subtree(tree.cast(), tree.root_node.cast());
    c_free!(tree);
}
