#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct _SetEntry {
    pub data: SetValue,
    pub next: Ptr<SetEntry>,
}
