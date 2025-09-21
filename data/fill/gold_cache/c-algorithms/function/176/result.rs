pub fn list_remove_entry(mut list: Ptr<Ptr<ListEntry>>, mut entry: Ptr<ListEntry>) -> i32 {
    if (list == NULL!() || *list == NULL!() || entry == NULL!()).as_bool() {
        return 0;
    }

    if (entry.prev == NULL!()).as_bool() {
        *list = entry.next.cast();

        if (entry.next != NULL!()).as_bool() {
            entry.next.prev = NULL!();
        }
    } else {
        entry.prev.next = entry.next.cast();

        if (entry.next != NULL!()).as_bool() {
            entry.next.prev = entry.prev.cast();
        }
    }

    c_free!(entry);

    return 1;
}
