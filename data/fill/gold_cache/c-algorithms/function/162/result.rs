pub fn sortedarray_insert(mut sortedarray: Ptr<SortedArray>, mut data: SortedArrayValue) -> i32 {
    let mut left: u32 = 0;
    let mut right: u32 = sortedarray.length;
    let mut index: u32 = 0;

    right = if right > 1 { right } else { 0 };

    while (left != right) {
        index = (left + right) / 2;

        let mut order: i32 = (sortedarray.cmp_func)(data, sortedarray.data[index]);
        if order < 0 {
            right = index;
        } else if order > 0 {
            left = (index + 1);
        } else {
            break;
        }
    }

    if (sortedarray.length > 0) && ((sortedarray.cmp_func)(data, sortedarray.data[index]) > 0) {
        index += 1;
    }

    if (sortedarray.length + 1 > sortedarray._alloced) {
        let mut newsize: u32;
        let mut data: Ptr<SortedArrayValue>;

        newsize = sortedarray._alloced * 2;
        data = c_realloc!(sortedarray.data, c_sizeof!(SortedArrayValue) * newsize);

        if (data == NULL!()) {
            return 0;
        } else {
            sortedarray.data = data;
            sortedarray._alloced = newsize;
        }
    }

    c_memmove!(
        c_ref!(sortedarray.data[index + 1]),
        c_ref!(sortedarray.data[index]),
        (sortedarray.length - index) * c_sizeof!(SortedArrayValue)
    );

    sortedarray.data[index] = data;
    sortedarray.length += 1;

    return 1;
}
