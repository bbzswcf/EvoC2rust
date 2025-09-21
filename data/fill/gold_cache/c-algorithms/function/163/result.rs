pub fn sortedarray_index_of(mut sortedarray: Ptr<SortedArray>, mut data: SortedArrayValue) -> i32 {
    if (sortedarray == NULL!()).as_bool() {
        return -1;
    }

    let mut left: u32 = 0;
    let mut right: u32 = sortedarray.length.cast();
    let mut index: u32 = 0;

    right = if right > 1 { right } else { 0 };

    while (left != right).as_bool() {
        index = (left + right) / 2;

        let mut order: i32 = (sortedarray.cmp_func)(data.cast(), sortedarray.data[index].cast()).cast();
        if order < 0 {
            right = index.cast();
        } else if order > 0 {
            left = (index + 1).cast();
        } else {
            left = sortedarray_first_index(sortedarray.cast(), data.cast(), left.cast(), index.cast()).cast();
            right = sortedarray_last_index(sortedarray.cast(), data.cast(), index.cast(), right.cast()).cast();

            c_for!(index = left; index <= right; index.suffix_plus_plus(); {
                if (sortedarray.equ_func)(data.cast(), sortedarray.data[index].cast()).as_bool() {
                    return index.cast::<i32>();
                }
            });

            return -1;
        }
    }

    return -1;
}
