pub fn list_free(mut list: Ptr<ListEntry>) {
    let mut entry: Ptr<ListEntry> = list.cast();

    while (entry != NULL!()).as_bool() {
        let mut next: Ptr<ListEntry> = entry.next.cast();

        c_free!(entry);

        entry = next.cast();
    }
}
