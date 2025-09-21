pub fn sortedarray_get(mut array: Ptr<SortedArray>, mut i: u32) -> SortedArrayValue {
    if (array == NULL!()).as_bool() {
        return NULL!();
    }
    return array.data[i];
}
