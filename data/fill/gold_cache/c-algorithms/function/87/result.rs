pub fn slist_prepend(mut list: Ptr<Ptr<SListEntry>>, mut data: SListValue) -> Ptr<SListEntry> {
    let mut newentry: Ptr<SListEntry> = c_malloc!(c_sizeof!(SListEntry));

    if (newentry == NULL!()) {
        return NULL!();
    }

    newentry.data = data;

    newentry.next = *list;
    *list = newentry;

    return newentry;
}
