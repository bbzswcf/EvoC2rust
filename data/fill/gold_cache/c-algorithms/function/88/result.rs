pub fn slist_append(mut list: Ptr<Ptr<SListEntry>>, mut data: SListValue) -> Ptr<SListEntry> {
    let mut rover: Ptr<SListEntry> = Default::default();
    let mut newentry: Ptr<SListEntry> = Default::default();

    newentry = c_malloc!(c_sizeof!(SListEntry));

    if (newentry == NULL!()).as_bool() {
        return NULL!();
    }

    newentry.data = data.cast();
    newentry.next = NULL!();

    if (*list == NULL!()).as_bool() {
        *list = newentry.cast();
    } else {
        rover = (*list).cast();
        while (rover.next != NULL!()).as_bool() {
            rover = rover.next.cast();
        }
        rover.next = newentry.cast();
    }

    return newentry.cast();
}
