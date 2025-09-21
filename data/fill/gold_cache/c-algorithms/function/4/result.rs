pub fn rb_tree_rotate(mut tree: Ptr<RBTree>, mut node: Ptr<RBTreeNode>, mut direction: RBTreeNodeSide) -> Ptr<RBTreeNode> {
    let mut new_root: Ptr<RBTreeNode>;

    new_root = node.children[1 - direction].cast();

    rb_tree_node_replace(tree.cast(), node.cast(), new_root.cast());

    node.children[1 - direction] = new_root.children[direction].cast();
    new_root.children[direction] = node.cast();

    node.parent = new_root.cast();

    if (node.children[1 - direction] != NULL!()).as_bool() {
        node.children[1 - direction].parent = node.cast();
    }

    return new_root.cast();
}
