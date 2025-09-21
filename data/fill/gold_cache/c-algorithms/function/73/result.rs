pub fn avl_tree_node_get_replacement(mut tree: Ptr<AVLTree>, mut node: Ptr<AVLTreeNode>) -> Ptr<AVLTreeNode> {
    let mut left_subtree: Ptr<AVLTreeNode> = Default::default();
    let mut right_subtree: Ptr<AVLTreeNode> = Default::default();
    let mut result: Ptr<AVLTreeNode> = Default::default();
    let mut child: Ptr<AVLTreeNode> = Default::default();
    let mut left_height: i32 = Default::default();
    let mut right_height: i32 = Default::default();
    let mut side: i32 = Default::default();

    left_subtree = node.children[AVL_TREE_NODE_LEFT!()].cast();
    right_subtree = node.children[AVL_TREE_NODE_RIGHT!()].cast();

    if (left_subtree == NULL!() && right_subtree == NULL!()).as_bool() {
        return NULL!();
    }

    left_height = avl_tree_subtree_height(left_subtree.cast()).cast();
    right_height = avl_tree_subtree_height(right_subtree.cast()).cast();

    if (left_height < right_height).as_bool() {
        side = AVL_TREE_NODE_RIGHT!();
    } else {
        side = AVL_TREE_NODE_LEFT!();
    }

    result = node.children[side].cast();

    while (result.children[1 - side] != NULL!()).as_bool() {
        result = result.children[1 - side].cast();
    }

    child = result.children[side].cast();
    avl_tree_node_replace(tree.cast(), result.cast(), child.cast());

    avl_tree_update_height(result.parent.cast());

    return result.cast();
}
