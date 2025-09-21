pub fn slist_data(mut listentry: Ptr<SListEntry>) -> SListValue {
    return listentry.data.cast();
}
