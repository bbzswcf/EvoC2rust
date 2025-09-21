pub fn avl_tree_node_key(mut node: Ptr<AVLTreeNode>) -> AVLTreeKey {
    return node.key.cast();
}
