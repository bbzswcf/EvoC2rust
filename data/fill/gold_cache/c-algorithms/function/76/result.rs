pub fn avl_tree_lookup_node(mut tree: Ptr<AVLTree>, mut key: AVLTreeKey) -> Ptr<AVLTreeNode> {
    let mut node: Ptr<AVLTreeNode> = Default::default();
    let mut diff: i32 = Default::default();

    node = tree.root_node.cast();

    while (node != NULL!()).as_bool() {
        diff = (tree.compare_func)(key.cast(), node.key.cast()).cast();

        if diff == 0 {
            return node.cast();
        } else if diff < 0 {
            node = node.children[AVL_TREE_NODE_LEFT!()].cast();
        } else {
            node = node.children[AVL_TREE_NODE_RIGHT!()].cast();
        }
    }

    return NULL!();
}
