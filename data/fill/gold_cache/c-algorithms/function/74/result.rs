pub fn avl_tree_remove_node(mut tree: Ptr<AVLTree>, mut node: Ptr<AVLTreeNode>) {
    let mut swap_node: Ptr<AVLTreeNode> = Default::default();
    let mut balance_startpoint: Ptr<AVLTreeNode> = Default::default();
    let mut i: i32 = Default::default();

    swap_node = avl_tree_node_get_replacement(tree, node);

    if (swap_node == NULL!()) {
        avl_tree_node_replace(tree, node, NULL!());

        balance_startpoint = node.parent;
    } else {
        if (swap_node.parent == node) {
            balance_startpoint = swap_node;
        } else {
            balance_startpoint = swap_node.parent;
        }

        c_for!(i = 0; i < 2; i.prefix_plus_plus(); {
            swap_node.children[i] = node.children[i];

            if (swap_node.children[i] != NULL!()) {
                swap_node.children[i].parent = swap_node;
            }
        });

        swap_node.height = node.height;

        avl_tree_node_replace(tree, node, swap_node);
    }

    c_free!(node);

    tree.num_nodes -= 1;

    avl_tree_balance_to_root(tree, balance_startpoint);
}
