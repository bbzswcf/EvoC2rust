pub fn list_iter_has_more(mut iter: Ptr<ListIterator>) -> i32 {
    if (iter.current == NULL!() || iter.current != *iter.prev_next).as_bool() {
        return (*iter.prev_next != NULL!()).cast();
    } else {
        return (iter.current.next != NULL!()).cast();
    }
}
