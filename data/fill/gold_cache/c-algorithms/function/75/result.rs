pub fn avl_tree_remove(mut tree: Ptr<AVLTree>, mut key: AVLTreeKey) -> i32 {
    let mut node: Ptr<AVLTreeNode> = Default::default();

    node = avl_tree_lookup_node(tree.cast(), key.cast());

    if (node == NULL!()).as_bool() {
        return 0;
    }

    avl_tree_remove_node(tree.cast(), node.cast());

    return 1;
}
