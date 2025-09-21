pub fn set_enlarge(mut set: Ptr<Set>) -> i32 {
    let mut rover: Ptr<SetEntry> = Default::default();
    let mut next: Ptr<SetEntry> = Default::default();
    let mut old_table: Ptr<Ptr<SetEntry>> = Default::default();
    let mut old_table_size: u32 = Default::default();
    let mut old_prime_index: u32 = Default::default();
    let mut index: u32 = Default::default();
    let mut i: u32 = Default::default();

    old_table = set.table.cast();
    old_table_size = set.table_size.cast();
    old_prime_index = set.prime_index.cast();

    set.prime_index.prefix_plus_plus();

    if !set_allocate_table(set.cast()).as_bool() {
        set.table = old_table.cast();
        set.table_size = old_table_size.cast();
        set.prime_index = old_prime_index.cast();

        return 0;
    }

    c_for!(i = 0; i < old_table_size; i.prefix_plus_plus(); {
        rover = old_table[i].cast();

        while (rover != NULL!()).as_bool() {
            next = rover.next.cast();

            index = (set.hash_func)(rover.data.cast()) % set.table_size;
            rover.next = set.table[index].cast();
            set.table[index] = rover.cast();

            rover = next.cast();
        }
    });

    c_free!(old_table);

    return 1;
}
