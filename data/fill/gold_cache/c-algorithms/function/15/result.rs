pub fn rb_tree_lookup(mut tree: Ptr<RBTree>, mut key: RBTreeKey) -> RBTreeValue {
    let mut node: Ptr<RBTreeNode> = Default::default();

    node = rb_tree_lookup_node(tree.cast(), key.cast());

    if (node == NULL!()).as_bool() {
        return RB_TREE_NULL!();
    } else {
        return node.value.cast();
    }
}
