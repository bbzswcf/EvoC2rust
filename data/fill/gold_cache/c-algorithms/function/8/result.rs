pub fn rb_tree_insert_case1(mut tree: Ptr<RBTree>, mut node: Ptr<RBTreeNode>) {
    if (node.parent == NULL!()).as_bool() {
        node.color = RB_TREE_NODE_BLACK!();
    } else {
        rb_tree_insert_case2(tree.cast(), node.cast());
    }
}
