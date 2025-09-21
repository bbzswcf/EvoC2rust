pub fn avl_tree_node_balance(mut tree: Ptr<AVLTree>, mut node: Ptr<AVLTreeNode>) -> Ptr<AVLTreeNode> {
    let mut left_subtree: Ptr<AVLTreeNode>;
    let mut right_subtree: Ptr<AVLTreeNode>;
    let mut child: Ptr<AVLTreeNode>;
    let mut diff: i32;

    left_subtree = node.children[AVL_TREE_NODE_LEFT!()].cast();
    right_subtree = node.children[AVL_TREE_NODE_RIGHT!()].cast();

    diff = (avl_tree_subtree_height(right_subtree.cast()) - avl_tree_subtree_height(left_subtree.cast())).cast();

    if (diff >= 2).as_bool() {
        child = right_subtree.cast();

        if (avl_tree_subtree_height(child.children[AVL_TREE_NODE_RIGHT!()].cast()) <
            avl_tree_subtree_height(child.children[AVL_TREE_NODE_LEFT!()].cast())).as_bool() {
            avl_tree_rotate(tree.cast(), right_subtree.cast(), AVL_TREE_NODE_RIGHT!().cast());
        }

        node = avl_tree_rotate(tree.cast(), node.cast(), AVL_TREE_NODE_LEFT!().cast()).cast();
    } else if (diff <= -2).as_bool() {
        child = node.children[AVL_TREE_NODE_LEFT!()].cast();

        if (avl_tree_subtree_height(child.children[AVL_TREE_NODE_LEFT!()].cast()) <
            avl_tree_subtree_height(child.children[AVL_TREE_NODE_RIGHT!()].cast())).as_bool() {
            avl_tree_rotate(tree.cast(), left_subtree.cast(), AVL_TREE_NODE_LEFT!().cast());
        }

        node = avl_tree_rotate(tree.cast(), node.cast(), AVL_TREE_NODE_RIGHT!().cast()).cast();
    }

    avl_tree_update_height(node.cast());

    return node.cast();
}
