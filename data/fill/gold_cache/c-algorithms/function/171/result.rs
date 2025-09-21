pub fn list_next(mut listentry: Ptr<ListEntry>) -> Ptr<ListEntry> {
    if (listentry == NULL!()).as_bool() {
        return NULL!();
    }

    return listentry.next.cast();
}
