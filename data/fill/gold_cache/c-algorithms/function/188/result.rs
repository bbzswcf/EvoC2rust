pub fn hash_table_free_entry(mut hash_table: Ptr<HashTable>, mut entry: Ptr<HashTableEntry>) {
    let mut pair: Ptr<HashTablePair> = c_ref!(entry.pair).cast();

    if (hash_table.key_free_func != NULL!()).as_bool() {
        (hash_table.key_free_func)(pair.key.cast());
    }

    if (hash_table.value_free_func != NULL!()).as_bool() {
        (hash_table.value_free_func)(pair.value.cast());
    }

    c_free!(entry);
}
