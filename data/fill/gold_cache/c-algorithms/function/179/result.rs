pub fn list_sort(mut list: Ptr<Ptr<ListEntry>>, mut compare_func: ListCompareFunc) {
    list_sort_internal(list.cast(), compare_func.cast());
}
