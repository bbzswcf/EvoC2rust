#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct _QueueEntry {
    pub data: QueueValue,
    pub prev: Ptr<QueueEntry>,
    pub next: Ptr<QueueEntry>,
}
