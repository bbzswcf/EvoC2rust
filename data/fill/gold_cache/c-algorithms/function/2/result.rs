pub fn rb_tree_node_uncle(mut node: Ptr<RBTreeNode>) -> Ptr<RBTreeNode> {
    return rb_tree_node_sibling(node.parent.cast()).cast();
}
