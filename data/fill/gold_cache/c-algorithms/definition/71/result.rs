#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct _ListEntry {
    pub data: ListValue,
    pub prev: Ptr<ListEntry>,
    pub next: Ptr<ListEntry>,
}
