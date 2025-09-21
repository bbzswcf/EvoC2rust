pub fn avl_tree_rotate(mut tree: Ptr<AVLTree>, mut node: Ptr<AVLTreeNode>, mut direction: AVLTreeNodeSide) -> Ptr<AVLTreeNode> {
    let mut new_root: Ptr<AVLTreeNode>;
    new_root = node.children[1 - direction].cast();
    avl_tree_node_replace(tree.cast(), node.cast(), new_root.cast());
    node.children[1 - direction] = new_root.children[direction].cast();
    new_root.children[direction] = node.cast();
    node.parent = new_root.cast();
    if (node.children[1 - direction] != NULL!()).as_bool() {
        node.children[1 - direction].parent = node.cast();
    }
    avl_tree_update_height(new_root.cast());
    avl_tree_update_height(node.cast());
    return new_root.cast();
}
