pub fn list_prepend(mut list: Ptr<Ptr<ListEntry>>, mut data: ListValue) -> Ptr<ListEntry> {
    let mut newentry: Ptr<ListEntry>;

    if (list == NULL!()).as_bool() {
        return NULL!();
    }

    newentry = c_malloc!(c_sizeof!(ListEntry));

    if (newentry == NULL!()).as_bool() {
        return NULL!();
    }

    newentry.data = data.cast();

    if (*list != NULL!()).as_bool() {
        (*list).prev = newentry.cast();
    }
    newentry.prev = NULL!();
    newentry.next = (*list).cast();
    *list = newentry.cast();

    return newentry.cast();
}
