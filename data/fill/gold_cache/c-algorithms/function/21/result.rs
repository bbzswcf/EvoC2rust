pub fn rb_tree_node_child(mut node: Ptr<RBTreeNode>, mut side: RBTreeNodeSide) -> Ptr<RBTreeNode> {
    if (side == RB_TREE_NODE_LEFT!() || side == RB_TREE_NODE_RIGHT!()).as_bool() {
        return node.children[side].cast();
    } else {
        return NULL!();
    }
}
