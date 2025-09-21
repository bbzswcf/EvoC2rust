pub fn bloom_filter_load(mut bloomfilter: Ptr<BloomFilter>, mut array: Ptr<u8>) {
    let mut array_size: u32;
    array_size = (bloomfilter.table_size + 7) / 8;
    c_memcpy!(bloomfilter.table, array, array_size);
}
