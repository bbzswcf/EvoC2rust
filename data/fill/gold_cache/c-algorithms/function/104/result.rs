pub fn slist_iter_remove(mut iter: Ptr<SListIterator>) {
    if (iter.current == NULL!() || iter.current != *iter.prev_next).as_bool() {
    } else {
        *iter.prev_next = iter.current.next.cast();
        c_free!(iter.current);
        iter.current = NULL!();
    }
}
