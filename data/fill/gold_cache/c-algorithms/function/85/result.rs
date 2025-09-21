pub fn avl_tree_to_array(mut tree: Ptr<AVLTree>) -> Ptr<AVLTreeValue> {
    let mut array: Ptr<AVLTreeValue>;
    let mut index: i32;

    array = c_malloc!(c_sizeof!(AVLTreeValue) * tree.num_nodes);

    if (array == NULL!()).as_bool() {
        return NULL!();
    }

    index = 0;

    avl_tree_to_array_add_subtree(tree.root_node.cast(), array.cast(), c_ref!(index).cast());

    return array.cast();
}
