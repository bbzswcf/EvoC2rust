pub fn sortedarray_remove(mut sortedarray: Ptr<SortedArray>, mut index: u32) {
    sortedarray_remove_range(sortedarray.cast(), index.cast(), 1);
}
