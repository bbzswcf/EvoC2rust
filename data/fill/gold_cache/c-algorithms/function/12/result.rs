pub fn rb_tree_insert_case5(mut tree: Ptr<RBTree>, mut node: Ptr<RBTreeNode>) {
    let mut parent: Ptr<RBTreeNode> = Default::default();
    let mut grandparent: Ptr<RBTreeNode> = Default::default();
    let mut side: RBTreeNodeSide = Default::default();

    parent = node.parent.cast();
    grandparent = parent.parent.cast();

    side = rb_tree_node_side(node.cast()).cast();

    rb_tree_rotate(tree.cast(), grandparent.cast(), (1 - side).cast());

    parent.color = RB_TREE_NODE_BLACK!();
    grandparent.color = RB_TREE_NODE_RED!();
}
