pub type HashTableEqualFunc = FuncPtr<fn(HashTableKey, HashTableKey) -> i32>;
