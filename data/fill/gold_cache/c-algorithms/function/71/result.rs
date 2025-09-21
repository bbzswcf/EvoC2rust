pub fn avl_tree_balance_to_root(mut tree: Ptr<AVLTree>, mut node: Ptr<AVLTreeNode>) {
    let mut rover: Ptr<AVLTreeNode> = node.cast();
    while (rover != NULL!()).as_bool() {
        rover = avl_tree_node_balance(tree.cast(), rover.cast()).cast();
        rover = rover.parent.cast();
    }
}
