pub fn set_new(mut hash_func: SetHashFunc, mut equal_func: SetEqualFunc) -> Ptr<Set> {
    let mut new_set: Ptr<Set> = c_malloc!(c_sizeof!(Set));

    if (new_set == NULL!()).as_bool() {
        return NULL!();
    }

    new_set.hash_func = hash_func.cast();
    new_set.equal_func = equal_func.cast();
    new_set.entries = 0;
    new_set.prime_index = 0;
    new_set.free_func = NULL!();

    if !set_allocate_table(new_set.cast()).as_bool() {
        c_free!(new_set);
        return NULL!();
    }

    return new_set.cast();
}
