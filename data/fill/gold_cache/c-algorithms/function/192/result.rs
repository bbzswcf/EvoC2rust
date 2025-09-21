pub fn hash_table_enlarge(mut hash_table: Ptr<HashTable>) -> i32 {
    let mut old_table: Ptr<Ptr<HashTableEntry>> = Default::default();
    let mut old_table_size: u32 = Default::default();
    let mut old_prime_index: u32 = Default::default();
    let mut rover: Ptr<HashTableEntry> = Default::default();
    let mut pair: Ptr<HashTablePair> = Default::default();
    let mut next: Ptr<HashTableEntry> = Default::default();
    let mut index: u32 = Default::default();
    let mut i: u32 = Default::default();

    old_table = hash_table.table.cast();
    old_table_size = hash_table.table_size.cast();
    old_prime_index = hash_table.prime_index.cast();

    hash_table.prime_index.prefix_plus_plus();

    if !hash_table_allocate_table(hash_table.cast()).as_bool() {
        hash_table.table = old_table.cast();
        hash_table.table_size = old_table_size.cast();
        hash_table.prime_index = old_prime_index.cast();

        return 0;
    }

    c_for!(i = 0; i < old_table_size; i.prefix_plus_plus(); {
        rover = old_table[i].cast();

        while (rover != NULL!()).as_bool() {
            next = rover.next.cast();

            pair = c_ref!(rover.pair).cast();

            index = (hash_table.hash_func)(pair.key.cast()) % hash_table.table_size;

            rover.next = hash_table.table[index].cast();
            hash_table.table[index] = rover.cast();

            rover = next.cast();
        }
    });

    c_free!(old_table);

    return 1;
}
