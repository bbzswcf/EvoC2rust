pub fn rb_tree_free_subtree(mut node: Ptr<RBTreeNode>) {
    if (node != NULL!()).as_bool() {
        rb_tree_free_subtree(node.children[RB_TREE_NODE_LEFT!()].cast());
        rb_tree_free_subtree(node.children[RB_TREE_NODE_RIGHT!()].cast());
        c_free!(node);
    }
}
