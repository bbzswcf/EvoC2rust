pub fn avl_tree_node_replace(mut tree: Ptr<AVLTree>, mut node1: Ptr<AVLTreeNode>, mut node2: Ptr<AVLTreeNode>) {
    let mut side: i32 = Default::default();

    if (node2 != NULL!()).as_bool() {
        node2.parent = node1.parent.cast();
    }

    if (node1.parent == NULL!()).as_bool() {
        tree.root_node = node2.cast();
    } else {
        side = avl_tree_node_parent_side(node1.cast()).cast();
        node1.parent.children[side] = node2.cast();

        avl_tree_update_height(node1.parent.cast());
    }
}
