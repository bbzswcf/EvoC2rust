pub fn bloom_filter_free(mut bloomfilter: Ptr<BloomFilter>) {
    c_free!(bloomfilter.table);
    c_free!(bloomfilter);
}
