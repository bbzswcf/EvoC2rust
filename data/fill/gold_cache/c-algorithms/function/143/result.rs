pub fn set_iter_next(mut iterator: Ptr<SetIterator>) -> SetValue {
    let mut set: Ptr<Set> = Default::default();
    let mut result: SetValue = Default::default();
    let mut current_entry: Ptr<SetEntry> = Default::default();
    let mut chain: u32 = Default::default();

    set = iterator.set;

    if (iterator.next_entry == NULL!()) {
        return SET_NULL!();
    }

    current_entry = iterator.next_entry;
    result = current_entry.data;

    if (current_entry.next != NULL!()) {
        iterator.next_entry = current_entry.next;
    } else {
        iterator.next_entry = NULL!();

        chain = (iterator.next_chain + 1);

        while (chain < set.table_size) {
            if (set.table[chain] != NULL!()) {
                iterator.next_entry = set.table[chain];

                break;
            }

            chain.prefix_plus_plus();
        }

        iterator.next_chain = chain;
    }

    return result;
}
