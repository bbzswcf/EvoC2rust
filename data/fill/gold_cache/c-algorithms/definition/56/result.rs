#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct _Set {
    pub table: Ptr<Ptr<SetEntry>>,
    pub entries: u32,
    pub table_size: u32,
    pub prime_index: u32,
    pub hash_func: SetHashFunc,
    pub equal_func: SetEqualFunc,
    pub free_func: SetFreeFunc,
}
