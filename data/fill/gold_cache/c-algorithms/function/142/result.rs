pub fn set_iterate(mut set: Ptr<Set>, mut iter: Ptr<SetIterator>) {
    let mut chain: u32 = Default::default();

    iter.set = set.cast();
    iter.next_entry = NULL!();

    c_for!(chain = 0; chain < set.table_size; chain.prefix_plus_plus(); {
        if (set.table[chain] != NULL!()).as_bool() {
            iter.next_entry = set.table[chain].cast();
            break;
        }
    });

    iter.next_chain = chain.cast();
}
