#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct _AVLTree {
    pub root_node: Ptr<AVLTreeNode>,
    pub compare_func: AVLTreeCompareFunc,
    pub num_nodes: u32,
}
