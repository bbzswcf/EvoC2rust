pub fn slist_next(mut listentry: Ptr<SListEntry>) -> Ptr<SListEntry> {
    return listentry.next.cast();
}
