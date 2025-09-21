pub fn list_iterate(mut list: Ptr<Ptr<ListEntry>>, mut iter: Ptr<ListIterator>) {
    iter.prev_next = list.cast();
    iter.current = NULL!();
}
