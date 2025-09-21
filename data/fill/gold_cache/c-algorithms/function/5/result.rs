pub fn rb_tree_new(mut compare_func: RBTreeCompareFunc) -> Ptr<RBTree> {
    let mut new_tree: Ptr<RBTree> = c_malloc!(c_sizeof!(RBTree));

    if (new_tree == NULL!()).as_bool() {
        return NULL!();
    }

    new_tree.root_node = NULL!();
    new_tree.num_nodes = 0;
    new_tree.compare_func = compare_func.cast();

    return new_tree.cast();
}
