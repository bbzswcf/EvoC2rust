pub fn hash_table_insert(mut hash_table: Ptr<HashTable>, mut key: HashTableKey, mut value: HashTableValue) -> i32 {
    let mut rover: Ptr<HashTableEntry> = Default::default();
    let mut pair: Ptr<HashTablePair> = Default::default();
    let mut newentry: Ptr<HashTableEntry> = Default::default();
    let mut index: u32 = Default::default();

    if ((hash_table.entries * 3) / hash_table.table_size > 0).as_bool() {
        if !hash_table_enlarge(hash_table.cast()).as_bool() {
            return 0;
        }
    }

    index = (hash_table.hash_func)(key.cast()) % hash_table.table_size;

    rover = hash_table.table[index].cast();

    while (rover != NULL!()).as_bool() {
        pair = c_ref!(rover.pair).cast();

        if (hash_table.equal_func)(pair.key.cast(), key.cast()) != 0 {
            if (hash_table.value_free_func != NULL!()).as_bool() {
                (hash_table.value_free_func)(pair.value.cast());
            }

            if (hash_table.key_free_func != NULL!()).as_bool() {
                (hash_table.key_free_func)(pair.key.cast());
            }

            pair.key = key.cast();
            pair.value = value.cast();

            return 1;
        }

        rover = rover.next.cast();
    }

    newentry = c_malloc!(c_sizeof!(HashTableEntry));

    if (newentry == NULL!()).as_bool() {
        return 0;
    }

    newentry.pair.key = key.cast();
    newentry.pair.value = value.cast();

    newentry.next = hash_table.table[index].cast();
    hash_table.table[index] = newentry.cast();

    hash_table.entries.prefix_plus_plus();

    return 1;
}
