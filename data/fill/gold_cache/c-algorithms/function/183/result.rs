pub fn list_iter_next(mut iter: Ptr<ListIterator>) -> ListValue {
    if (iter.current == NULL!() || iter.current != *iter.prev_next).as_bool() {
        iter.current = *iter.prev_next;
    } else {
        iter.prev_next = c_ref!(iter.current.next).cast();
        iter.current = iter.current.next;
    }

    if (iter.current == NULL!()).as_bool() {
        return LIST_NULL!();
    } else {
        return iter.current.data;
    }
}
