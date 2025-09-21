pub fn avl_tree_node_parent(mut node: Ptr<AVLTreeNode>) -> Ptr<AVLTreeNode> {
    return node.parent.cast();
}
