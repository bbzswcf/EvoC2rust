pub fn avl_tree_node_parent_side(mut node: Ptr<AVLTreeNode>) -> AVLTreeNodeSide {
    if (node.parent.children[AVL_TREE_NODE_LEFT!()] == node).as_bool() {
        return AVL_TREE_NODE_LEFT!();
    } else {
        return AVL_TREE_NODE_RIGHT!();
    }
}
