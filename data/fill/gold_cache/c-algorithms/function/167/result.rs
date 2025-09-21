pub fn list_append(mut list: Ptr<Ptr<ListEntry>>, mut data: ListValue) -> Ptr<ListEntry> {
    let mut rover: Ptr<ListEntry> = Default::default();
    let mut newentry: Ptr<ListEntry> = Default::default();

    if (list == NULL!()) {
        return NULL!();
    }

    newentry = c_malloc!(c_sizeof!(ListEntry));

    if (newentry == NULL!()) {
        return NULL!();
    }

    newentry.data = data;
    newentry.next = NULL!();

    if (*list == NULL!()) {
        *list = newentry;
        newentry.prev = NULL!();
    } else {
        rover = *list;
        while (rover.next != NULL!()) {
            rover = rover.next;
        }

        newentry.prev = rover;
        rover.next = newentry;
    }

    return newentry;
}
