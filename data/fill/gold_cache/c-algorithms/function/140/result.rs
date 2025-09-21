pub fn set_union(mut set1: Ptr<Set>, mut set2: Ptr<Set>) -> Ptr<Set> {
    let mut iterator: SetIterator = Default::default();
    let mut new_set: Ptr<Set>;
    let mut value: SetValue = Default::default();

    new_set = set_new(set1.hash_func, set1.equal_func);

    if (new_set == NULL!()).as_bool() {
        return NULL!();
    }

    set_iterate(set1.cast(), c_ref!(iterator).cast());

    while set_iter_has_more(c_ref!(iterator).cast()).as_bool() {
        value = set_iter_next(c_ref!(iterator).cast());

        if !set_insert(new_set.cast(), value.cast()).as_bool() {
            set_free(new_set.cast());
            return NULL!();
        }
    }

    set_iterate(set2.cast(), c_ref!(iterator).cast());

    while set_iter_has_more(c_ref!(iterator).cast()).as_bool() {
        value = set_iter_next(c_ref!(iterator).cast());

        if set_query(new_set.cast(), value.cast()) == 0 {
            if !set_insert(new_set.cast(), value.cast()).as_bool() {
                set_free(new_set.cast());
                return NULL!();
            }
        }
    }

    return new_set.cast();
}
