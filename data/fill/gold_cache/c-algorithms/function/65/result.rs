pub fn avl_tree_subtree_height(mut node: Ptr<AVLTreeNode>) -> i32 {
    if (node == NULL!()).as_bool() {
        return 0;
    } else {
        return node.height.cast();
    }
}
