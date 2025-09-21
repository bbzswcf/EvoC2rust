pub fn avl_tree_node_value(mut node: Ptr<AVLTreeNode>) -> AVLTreeValue {
    return node.value.cast();
}
