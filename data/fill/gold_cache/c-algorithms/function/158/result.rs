pub fn sortedarray_new(mut length: u32, mut equ_func: SortedArrayEqualFunc, mut cmp_func: SortedArrayCompareFunc) -> Ptr<SortedArray> {
    if (equ_func == NULL!() || cmp_func == NULL!()).as_bool() {
        return NULL!();
    }
    if (length == 0).as_bool() {
        length = 16;
    }
    let mut array: Ptr<SortedArrayValue> = c_malloc!(c_sizeof!(SortedArrayValue) * length);
    if (array == NULL!()).as_bool() {
        return NULL!();
    }
    let mut sortedarray: Ptr<SortedArray> = c_malloc!(c_sizeof!(SortedArray));
    if (sortedarray == NULL!()).as_bool() {
        c_free!(array);
        return NULL!();
    }
    sortedarray.data = array.cast();
    sortedarray.length = 0;
    sortedarray._alloced = length;
    sortedarray.equ_func = equ_func.cast();
    sortedarray.cmp_func = cmp_func.cast();
    return sortedarray.cast();
}
