pub fn slist_iter_next(mut iter: Ptr<SListIterator>) -> SListValue {
    if (iter.current == NULL!() || iter.current != *iter.prev_next).as_bool() {
        iter.current = *iter.prev_next;
    } else {
        iter.prev_next = c_ref!(iter.current.next).cast();
        iter.current = iter.current.next;
    }

    if (iter.current == NULL!()).as_bool() {
        return SLIST_NULL!();
    } else {
        return iter.current.data;
    }
}
