pub fn avl_tree_insert(mut tree: Ptr<AVLTree>, mut key: AVLTreeKey, mut value: AVLTreeValue) -> Ptr<AVLTreeNode> {
    let mut rover: Ptr<Ptr<AVLTreeNode>> = Default::default();
    let mut new_node: Ptr<AVLTreeNode> = Default::default();
    let mut previous_node: Ptr<AVLTreeNode> = Default::default();

    rover = c_ref!(tree.root_node).cast();
    previous_node = NULL!();

    while (*rover != NULL!()).as_bool() {
        previous_node = (*rover).cast();
        if ((tree.compare_func)(key.cast(), (*rover).key.cast()) < 0).as_bool() {
            rover = c_ref!((*rover).children[AVL_TREE_NODE_LEFT!()]).cast();
        } else {
            rover = c_ref!((*rover).children[AVL_TREE_NODE_RIGHT!()]).cast();
        }
    }

    new_node = c_malloc!(c_sizeof!(AVLTreeNode));

    if (new_node == NULL!()).as_bool() {
        return NULL!();
    }

    new_node.children[AVL_TREE_NODE_LEFT!()] = NULL!();
    new_node.children[AVL_TREE_NODE_RIGHT!()] = NULL!();
    new_node.parent = previous_node.cast();
    new_node.key = key.cast();
    new_node.value = value.cast();
    new_node.height = 1;

    *rover = new_node.cast();

    avl_tree_balance_to_root(tree.cast(), previous_node.cast());

    tree.num_nodes.prefix_plus_plus();

    return new_node.cast();
}
