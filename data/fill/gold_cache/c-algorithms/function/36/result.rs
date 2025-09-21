pub fn arraylist_sort(mut arraylist: Ptr<ArrayList>, mut compare_func: ArrayListCompareFunc) {
    arraylist_sort_internal(arraylist.data.cast(), arraylist.length.cast(), compare_func.cast());
}
