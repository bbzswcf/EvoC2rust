pub fn list_set_data(mut listentry: Ptr<ListEntry>, mut value: ListValue) {
    if (listentry != NULL!()).as_bool() {
        listentry.data = value.cast();
    }
}
