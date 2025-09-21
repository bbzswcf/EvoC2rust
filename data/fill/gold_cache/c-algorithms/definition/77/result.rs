pub type RBTreeNodeColor = i32;
macro_rules! RB_TREE_NODE_RED { () => { 0 } }
pub(crate) use RB_TREE_NODE_RED;
macro_rules! RB_TREE_NODE_BLACK { () => { 1 } }
pub(crate) use RB_TREE_NODE_BLACK;
