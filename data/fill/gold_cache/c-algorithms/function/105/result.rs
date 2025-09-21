pub fn bloom_filter_new(mut table_size: u32, mut hash_func: BloomFilterHashFunc, mut num_functions: u32) -> Ptr<BloomFilter> {
    let mut filter: Ptr<BloomFilter>;

    if (num_functions > salts.len() as u32).as_bool() {
        return NULL!();
    }

    filter = c_malloc!(c_sizeof!(BloomFilter));

    if (filter == NULL!()).as_bool() {
        return NULL!();
    }

    filter.table = c_calloc!((table_size + 7) / 8, 1);

    if (filter.table == NULL!()).as_bool() {
        c_free!(filter);
        return NULL!();
    }

    filter.hash_func = hash_func.cast();
    filter.num_functions = num_functions.cast();
    filter.table_size = table_size.cast();

    return filter.cast();
}
