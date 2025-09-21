pub fn set_intersection(mut set1: Ptr<Set>, mut set2: Ptr<Set>) -> Ptr<Set> {
    let mut new_set: Ptr<Set> = Default::default();
    let mut iterator: SetIterator = Default::default();
    let mut value: SetValue = Default::default();

    new_set = set_new(set1.hash_func, set2.equal_func).cast();

    if (new_set == NULL!()).as_bool() {
        return NULL!();
    }

    set_iterate(set1.cast(), c_ref!(iterator).cast());

    while set_iter_has_more(c_ref!(iterator).cast()).as_bool() {
        value = set_iter_next(c_ref!(iterator).cast()).cast();

        if (set_query(set2.cast(), value.cast()) != 0).as_bool() {
            if !set_insert(new_set.cast(), value.cast()).as_bool() {
                set_free(new_set.cast());

                return NULL!();
            }
        }
    }

    return new_set.cast();
}
