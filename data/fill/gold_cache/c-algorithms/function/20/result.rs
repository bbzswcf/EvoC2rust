pub fn rb_tree_node_value(mut node: Ptr<RBTreeNode>) -> RBTreeValue {
    return node.value.cast();
}
