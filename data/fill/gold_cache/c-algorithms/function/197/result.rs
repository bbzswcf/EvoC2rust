pub fn hash_table_iterate(mut hash_table: Ptr<HashTable>, mut iterator: Ptr<HashTableIterator>) {
    let mut chain: u32 = Default::default();

    iterator.hash_table = hash_table.cast();

    iterator.next_entry = NULL!();

    c_for!(chain = 0; chain < hash_table.table_size; chain.prefix_plus_plus(); {
        if (hash_table.table[chain] != NULL!()).as_bool() {
            iterator.next_entry = hash_table.table[chain].cast();
            iterator.next_chain = chain.cast();
            break;
        }
    });
}
