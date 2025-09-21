pub fn list_iter_remove(mut iter: Ptr<ListIterator>) {
    if (iter.current == NULL!()).as_bool() || (iter.current != *iter.prev_next).as_bool() {
    } else {
        *iter.prev_next = iter.current.next.cast();
        if (iter.current.next != NULL!()).as_bool() {
            iter.current.next.prev = iter.current.prev.cast();
        }
        c_free!(iter.current);
        iter.current = NULL!();
    }
}
