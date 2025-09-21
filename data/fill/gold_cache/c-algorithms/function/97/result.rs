pub fn slist_remove_data(mut list: Ptr<Ptr<SListEntry>>, mut callback: SListEqualFunc, mut data: SListValue) -> u32 {
    let mut rover: Ptr<Ptr<SListEntry>> = Default::default();
    let mut next: Ptr<SListEntry> = Default::default();
    let mut entries_removed: u32 = 0;

    rover = list.cast();

    while (*rover != NULL!()).as_bool() {
        if (callback((*rover).data.cast(), data.cast()) != 0).as_bool() {
            next = (*rover).next.cast();
            c_free!(*rover);
            *rover = next.cast();

            entries_removed.prefix_plus_plus();
        } else {
            rover = c_ref!((*rover).next).cast();
        }
    }

    return entries_removed.cast();
}
