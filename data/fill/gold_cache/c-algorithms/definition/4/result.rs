#[repr(C)]
#[derive(Default)]
pub struct _RBTreeNode {
    pub color: RBTreeNodeColor,
    pub key: RBTreeKey,
    pub value: RBTreeValue,
    pub parent: Ptr<RBTreeNode>,
    pub children: Array<Ptr<RBTreeNode>, 2>,
}
