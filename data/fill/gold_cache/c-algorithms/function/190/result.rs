pub fn hash_table_free(mut hash_table: Ptr<HashTable>) {
    let mut rover: Ptr<HashTableEntry> = Default::default();
    let mut next: Ptr<HashTableEntry> = Default::default();
    let mut i: u32 = Default::default();

    c_for!(let mut i = 0; i < hash_table.table_size; i.prefix_plus_plus(); {
        rover = hash_table.table[i].cast();
        while (rover != NULL!()).as_bool() {
            next = rover.next.cast();
            hash_table_free_entry(hash_table.cast(), rover.cast());
            rover = next.cast();
        }
    });

    c_free!(hash_table.table);
    c_free!(hash_table);
}
