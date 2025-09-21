pub fn rb_tree_lookup_node(mut tree: Ptr<RBTree>, mut key: RBTreeKey) -> Ptr<RBTreeNode> {
    let mut node: Ptr<RBTreeNode> = Default::default();
    let mut side: RBTreeNodeSide = Default::default();
    let mut diff: i32 = Default::default();

    node = tree.root_node.cast();

    while (node != NULL!()).as_bool() {
        diff = (tree.compare_func)(key.cast(), node.key.cast()).cast();

        if diff == 0 {
            return node.cast();
        } else if diff < 0 {
            side = RB_TREE_NODE_LEFT!();
        } else {
            side = RB_TREE_NODE_RIGHT!();
        }

        node = node.children[side].cast();
    }

    return NULL!();
}
