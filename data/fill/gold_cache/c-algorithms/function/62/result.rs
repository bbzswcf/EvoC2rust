pub fn avl_tree_new(mut compare_func: AVLTreeCompareFunc) -> Ptr<AVLTree> {
    let mut new_tree: Ptr<AVLTree> = c_malloc!(c_sizeof!(AVLTree));

    if (new_tree == NULL!()).as_bool() {
        return NULL!();
    }

    new_tree.root_node = NULL!();
    new_tree.compare_func = compare_func.cast();
    new_tree.num_nodes = 0;

    return new_tree.cast();
}
