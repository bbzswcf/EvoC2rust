pub fn avl_tree_lookup(mut tree: Ptr<AVLTree>, mut key: AVLTreeKey) -> AVLTreeValue {
    let mut node: Ptr<AVLTreeNode> = Default::default();

    node = avl_tree_lookup_node(tree.cast(), key.cast()).cast();

    if (node == NULL!()).as_bool() {
        return AVL_TREE_NULL!();
    } else {
        return node.value.cast();
    }
}
