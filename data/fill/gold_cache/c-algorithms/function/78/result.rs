pub fn avl_tree_root_node(mut tree: Ptr<AVLTree>) -> Ptr<AVLTreeNode> {
    return tree.root_node.cast();
}
