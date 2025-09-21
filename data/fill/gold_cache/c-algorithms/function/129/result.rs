pub fn set_allocate_table(mut set: Ptr<Set>) -> i32 {
    if (set.prime_index < set_num_primes).as_bool() {
        set.table_size = set_primes[set.prime_index].cast();
    } else {
        set.table_size = (set.entries * 10).cast();
    }
    set.table = c_calloc!(set.table_size, c_sizeof!(Ptr<SetEntry>));
    return (set.table != NULL!()).cast::<i32>();
}
