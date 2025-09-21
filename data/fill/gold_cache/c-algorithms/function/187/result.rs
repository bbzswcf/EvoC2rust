pub fn hash_table_allocate_table(mut hash_table: Ptr<HashTable>) -> i32 {
    let mut new_table_size: u32;

    if (hash_table.prime_index < hash_table_num_primes).as_bool() {
        new_table_size = hash_table_primes[hash_table.prime_index].cast();
    } else {
        new_table_size = (hash_table.entries * 10).cast();
    }

    hash_table.table_size = new_table_size.cast();

    hash_table.table = c_calloc!(hash_table.table_size, c_sizeof!(Ptr<HashTableEntry>));

    return (hash_table.table != NULL!()).cast::<i32>();
}
