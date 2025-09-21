#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct _Queue {
    pub head: Ptr<QueueEntry>,
    pub tail: Ptr<QueueEntry>,
}
