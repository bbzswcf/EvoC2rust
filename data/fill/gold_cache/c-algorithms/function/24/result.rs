pub fn rb_tree_num_entries(mut tree: Ptr<RBTree>) -> i32 {
    return tree.num_nodes.cast();
}
