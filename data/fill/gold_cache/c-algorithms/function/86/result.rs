pub fn slist_free(mut list: Ptr<SListEntry>) {
    let mut entry: Ptr<SListEntry> = list.cast();

    while (entry != NULL!()).as_bool() {
        let mut next: Ptr<SListEntry> = entry.next.cast();

        c_free!(entry);

        entry = next.cast();
    }
}
