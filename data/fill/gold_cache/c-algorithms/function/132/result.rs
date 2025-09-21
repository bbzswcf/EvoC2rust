pub fn set_free(mut set: Ptr<Set>) {
    let mut rover: Ptr<SetEntry> = Default::default();
    let mut next: Ptr<SetEntry> = Default::default();
    let mut i: u32 = Default::default();

    c_for!(i = 0; i < set.table_size; i.prefix_plus_plus(); {
        rover = set.table[i].cast();

        while (rover != NULL!()).as_bool() {
            next = rover.next.cast();

            set_free_entry(set.cast(), rover.cast());

            rover = next.cast();
        }
    });

    c_free!(set.table);

    c_free!(set);
}
