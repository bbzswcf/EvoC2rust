pub fn slist_remove_entry(mut list: Ptr<Ptr<SListEntry>>, mut entry: Ptr<SListEntry>) -> i32 {
    let mut rover: Ptr<SListEntry> = Default::default();

    if (*list == NULL!() || entry == NULL!()).as_bool() {
        return 0;
    }

    if (*list == entry).as_bool() {
        *list = entry.next.cast();
    } else {
        rover = (*list).cast();

        while (rover != NULL!() && rover.next != entry).as_bool() {
            rover = rover.next.cast();
        }

        if (rover == NULL!()).as_bool() {
            return 0;
        } else {
            rover.next = entry.next.cast();
        }
    }

    c_free!(entry);

    return 1;
}
