pub fn hash_table_num_entries(mut hash_table: Ptr<HashTable>) -> u32 {
    return hash_table.entries.cast();
}
