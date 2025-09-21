pub fn rb_tree_root_node(mut tree: Ptr<RBTree>) -> Ptr<RBTreeNode> {
    return tree.root_node.cast();
}
