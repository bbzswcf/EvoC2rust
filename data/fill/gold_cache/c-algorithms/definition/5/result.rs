#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct _RBTree {
    pub root_node: Ptr<RBTreeNode>,
    pub compare_func: RBTreeCompareFunc,
    pub num_nodes: i32,
}
