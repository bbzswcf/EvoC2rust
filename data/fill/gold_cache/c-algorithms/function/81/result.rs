pub fn avl_tree_node_child(mut node: Ptr<AVLTreeNode>, mut side: AVLTreeNodeSide) -> Ptr<AVLTreeNode> {
    if (side == AVL_TREE_NODE_LEFT!() || side == AVL_TREE_NODE_RIGHT!()).as_bool() {
        return node.children[side].cast();
    } else {
        return NULL!();
    }
}
