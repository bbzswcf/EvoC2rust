pub fn avl_tree_to_array_add_subtree(mut subtree: Ptr<AVLTreeNode>, mut array: Ptr<AVLTreeValue>, mut index: Ptr<i32>) {
    if (subtree == NULL!()).as_bool() {
        return;
    }

    avl_tree_to_array_add_subtree(subtree.children[AVL_TREE_NODE_LEFT!()].cast(), array.cast(), index.cast());

    array[*index] = subtree.key.cast();
    (*index).prefix_plus_plus();

    avl_tree_to_array_add_subtree(subtree.children[AVL_TREE_NODE_RIGHT!()].cast(), array.cast(), index.cast());
}
