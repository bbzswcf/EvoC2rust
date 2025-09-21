pub fn slist_sort(mut list: Ptr<Ptr<SListEntry>>, mut compare_func: SListCompareFunc) {
    slist_sort_internal(list.cast(), compare_func.cast());
}
