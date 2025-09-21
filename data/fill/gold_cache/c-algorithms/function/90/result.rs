pub fn slist_set_data(mut listentry: Ptr<SListEntry>, mut data: SListValue) {
    if (listentry != NULL!()).as_bool() {
        listentry.data = data.cast();
    }
}
