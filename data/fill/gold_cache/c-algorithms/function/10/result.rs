pub fn rb_tree_insert_case3(mut tree: Ptr<RBTree>, mut node: Ptr<RBTreeNode>) {
    let mut grandparent: Ptr<RBTreeNode> = Default::default();
    let mut uncle: Ptr<RBTreeNode> = Default::default();

    grandparent = node.parent.parent.cast();
    uncle = rb_tree_node_uncle(node.cast()).cast();

    if (uncle != NULL!() && uncle.color == RB_TREE_NODE_RED!()).as_bool() {
        node.parent.color = RB_TREE_NODE_BLACK!();
        uncle.color = RB_TREE_NODE_BLACK!();
        grandparent.color = RB_TREE_NODE_RED!();

        rb_tree_insert_case1(tree.cast(), grandparent.cast());
    } else {
        rb_tree_insert_case4(tree.cast(), node.cast());
    }
}
