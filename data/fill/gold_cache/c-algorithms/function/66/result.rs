pub fn avl_tree_update_height(mut node: Ptr<AVLTreeNode>) {
    let mut left_subtree: Ptr<AVLTreeNode> = Default::default();
    let mut right_subtree: Ptr<AVLTreeNode> = Default::default();
    let mut left_height: i32 = Default::default();
    let mut right_height: i32 = Default::default();

    left_subtree = node.children[AVL_TREE_NODE_LEFT!()].cast();
    right_subtree = node.children[AVL_TREE_NODE_RIGHT!()].cast();
    left_height = avl_tree_subtree_height(left_subtree.cast()).cast();
    right_height = avl_tree_subtree_height(right_subtree.cast()).cast();

    if (left_height > right_height).as_bool() {
        node.height = (left_height + 1).cast();
    } else {
        node.height = (right_height + 1).cast();
    }
}
