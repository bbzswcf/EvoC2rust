#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct _SListEntry {
    pub data: SListValue,
    pub next: Ptr<SListEntry>,
}
