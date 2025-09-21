pub fn avl_tree_free_subtree(mut tree: Ptr<AVLTree>, mut node: Ptr<AVLTreeNode>) {
    if (node == NULL!()).as_bool() {
        return;
    }

    avl_tree_free_subtree(tree.cast(), node.children[AVL_TREE_NODE_LEFT!()].cast());
    avl_tree_free_subtree(tree.cast(), node.children[AVL_TREE_NODE_RIGHT!()].cast());

    c_free!(node);
}
