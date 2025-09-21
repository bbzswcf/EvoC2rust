pub fn rb_tree_insert_case2(mut tree: Ptr<RBTree>, mut node: Ptr<RBTreeNode>) {
    if (node.parent.color != RB_TREE_NODE_BLACK!()).as_bool() {
        rb_tree_insert_case3(tree.cast(), node.cast());
    }
}
