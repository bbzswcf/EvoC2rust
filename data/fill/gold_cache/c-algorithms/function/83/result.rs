pub fn avl_tree_num_entries(mut tree: Ptr<AVLTree>) -> u32 {
    return tree.num_nodes.cast();
}
