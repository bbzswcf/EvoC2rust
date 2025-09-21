#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct _SListIterator {
    pub prev_next: Ptr<Ptr<SListEntry>>,
    pub current: Ptr<SListEntry>,
}
