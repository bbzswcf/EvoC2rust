pub fn rb_tree_insert_case4(mut tree: Ptr<RBTree>, mut node: Ptr<RBTreeNode>) {
    let mut next_node: Ptr<RBTreeNode> = Default::default();
    let mut side: RBTreeNodeSide = Default::default();

    side = rb_tree_node_side(node.cast()).cast();

    if (side != rb_tree_node_side(node.parent).cast()).as_bool() {
        next_node = node.parent.cast();

        rb_tree_rotate(tree.cast(), node.parent.cast(), (1 - side).cast());
    } else {
        next_node = node.cast();
    }

    rb_tree_insert_case5(tree.cast(), next_node.cast());
}
