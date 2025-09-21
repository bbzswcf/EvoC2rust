#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct _ListIterator {
    pub prev_next: Ptr<Ptr<ListEntry>>,
    pub current: Ptr<ListEntry>,
}
