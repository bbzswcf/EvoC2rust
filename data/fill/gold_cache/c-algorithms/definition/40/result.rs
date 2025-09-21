#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct _BloomFilter {
    pub hash_func: BloomFilterHashFunc,
    pub table: Ptr<u8>,
    pub table_size: u32,
    pub num_functions: u32,
}
