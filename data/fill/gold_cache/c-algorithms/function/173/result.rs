pub fn list_nth_data(mut list: Ptr<ListEntry>, mut n: u32) -> ListValue {
    let mut entry: Ptr<ListEntry> = Default::default();

    entry = list_nth_entry(list.cast(), n.cast());

    if (entry == NULL!()).as_bool() {
        return LIST_NULL!();
    } else {
        return entry.data.cast();
    }
}
