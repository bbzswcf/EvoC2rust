pub fn list_nth_entry(mut list: Ptr<ListEntry>, mut n: u32) -> Ptr<ListEntry> {
    let mut entry: Ptr<ListEntry> = Default::default();
    let mut i: u32 = Default::default();

    entry = list.cast();

    c_for!(i = 0; i < n; i.prefix_plus_plus(); {
        if (entry == NULL!()).as_bool() {
            return NULL!();
        }
        entry = entry.next.cast();
    });

    return entry.cast();
}
