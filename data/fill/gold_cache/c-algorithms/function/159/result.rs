pub fn sortedarray_free(mut sortedarray: Ptr<SortedArray>) {
    if (sortedarray != NULL!()).as_bool() {
        c_free!(sortedarray.data);
        c_free!(sortedarray);
    }
}
