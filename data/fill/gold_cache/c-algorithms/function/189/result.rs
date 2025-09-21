pub fn hash_table_new(mut hash_func: HashTableHashFunc, mut equal_func: HashTableEqualFunc) -> Ptr<HashTable> {
    let mut hash_table: Ptr<HashTable> = c_malloc!(c_sizeof!(HashTable));

    if (hash_table == NULL!()).as_bool() {
        return NULL!();
    }

    hash_table.hash_func = hash_func.cast();
    hash_table.equal_func = equal_func.cast();
    hash_table.key_free_func = NULL!();
    hash_table.value_free_func = NULL!();
    hash_table.entries = 0;
    hash_table.prime_index = 0;

    if !hash_table_allocate_table(hash_table.cast()).as_bool() {
        c_free!(hash_table);

        return NULL!();
    }

    return hash_table.cast();
}
