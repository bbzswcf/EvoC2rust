pub fn list_remove_data(mut list: Ptr<Ptr<ListEntry>>, mut callback: ListEqualFunc, mut data: ListValue) -> u32 {
    let mut entries_removed: u32 = 0;
    let mut rover: Ptr<ListEntry> = Default::default();
    let mut next: Ptr<ListEntry> = Default::default();

    if (list == NULL!() || callback == NULL!()).as_bool() {
        return 0;
    }

    entries_removed = 0;

    rover = *list;

    while (rover != NULL!()).as_bool() {
        next = rover.next;

        if (callback(rover.data.cast(), data.cast())).as_bool() {
            if (rover.prev == NULL!()).as_bool() {
                *list = rover.next.cast();
            } else {
                rover.prev.next = rover.next.cast();
            }

            if (rover.next != NULL!()).as_bool() {
                rover.next.prev = rover.prev.cast();
            }

            c_free!(rover);

            entries_removed.prefix_plus_plus();
        }

        rover = next.cast();
    }

    return entries_removed.cast();
}
