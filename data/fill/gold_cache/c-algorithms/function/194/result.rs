pub fn hash_table_lookup(mut hash_table: Ptr<HashTable>, mut key: HashTableKey) -> HashTableValue {
    let mut rover: Ptr<HashTableEntry> = Default::default();
    let mut pair: Ptr<HashTablePair> = Default::default();
    let mut index: u32 = Default::default();

    index = ((hash_table.hash_func)(key.cast()) % hash_table.table_size).cast();

    rover = hash_table.table[index].cast();

    while (rover != NULL!()).as_bool() {
        pair = c_ref!(rover.pair).cast();

        if ((hash_table.equal_func)(key.cast(), pair.key.cast()) != 0).as_bool() {
            return pair.value.cast();
        }

        rover = rover.next.cast();
    }

    return HASH_TABLE_NULL!();
}
