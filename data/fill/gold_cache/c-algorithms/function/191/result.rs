pub fn hash_table_register_free_functions(mut hash_table: Ptr<HashTable>, mut key_free_func: HashTableKeyFreeFunc, mut value_free_func: HashTableValueFreeFunc) {
    hash_table.key_free_func = key_free_func.cast();
    hash_table.value_free_func = value_free_func.cast();
}
