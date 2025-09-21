pub fn rb_tree_remove(mut tree: Ptr<RBTree>, mut key: RBTreeKey) -> i32 {
    let mut node: Ptr<RBTreeNode> = Default::default();

    node = rb_tree_lookup_node(tree.cast(), key.cast()).cast();

    if (node == NULL!()).as_bool() {
        return 0;
    }

    rb_tree_remove_node(tree.cast(), node.cast());

    return 1;
}
