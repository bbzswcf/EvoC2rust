pub fn slist_iterate(mut list: Ptr<Ptr<SListEntry>>, mut iter: Ptr<SListIterator>) {
    iter.prev_next = list.cast();
    iter.current = NULL!();
}
