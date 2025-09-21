pub fn rb_tree_insert(mut tree: Ptr<RBTree>, mut key: RBTreeKey, mut value: RBTreeValue) -> Ptr<RBTreeNode> {
    let mut node: Ptr<RBTreeNode>;
    let mut rover: Ptr<Ptr<RBTreeNode>>;
    let mut parent: Ptr<RBTreeNode>;
    let mut side: RBTreeNodeSide;

    node = c_malloc!(c_sizeof!(RBTreeNode));

    if (node == NULL!()) {
        return NULL!();
    }

    node.key = key;
    node.value = value;
    node.color = RB_TREE_NODE_RED!();
    node.children[RB_TREE_NODE_LEFT!()] = NULL!();
    node.children[RB_TREE_NODE_RIGHT!()] = NULL!();

    parent = NULL!();
    rover = c_ref!(tree.root_node);

    while (*rover != NULL!()) {
        parent = *rover;

        if ((tree.compare_func)(value, (*rover).value) < 0) {
            side = RB_TREE_NODE_LEFT!();
        } else {
            side = RB_TREE_NODE_RIGHT!();
        }

        rover = c_ref!((*rover).children[side]);
    }

    *rover = node;
    node.parent = parent;

    rb_tree_insert_case1(tree, node);

    tree.num_nodes.prefix_plus_plus();

    return node;
}
