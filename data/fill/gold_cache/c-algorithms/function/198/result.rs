pub fn hash_table_iter_has_more(mut iterator: Ptr<HashTableIterator>) -> i32 {
    return (iterator.next_entry != NULL!()).as_bool().cast();
}
