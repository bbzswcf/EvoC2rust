pub fn arraylist_sort_internal(mut list_data: Ptr<ArrayListValue>, mut list_length: u32, mut compare_func: ArrayListCompareFunc) {
    let mut pivot: ArrayListValue = Default::default();
    let mut tmp: ArrayListValue = Default::default();
    let mut i: u32 = Default::default();
    let mut list1_length: u32 = Default::default();
    let mut list2_length: u32 = Default::default();

    if (list_length <= 1).as_bool() {
        return;
    }

    pivot = list_data[list_length - 1].cast();

    list1_length = 0;

    c_for!(i = 0; i < list_length - 1; i.prefix_plus_plus(); {
        if (compare_func(list_data[i].cast(), pivot.cast()) < 0).as_bool() {
            tmp = list_data[i].cast();
            list_data[i] = list_data[list1_length].cast();
            list_data[list1_length] = tmp.cast();

            list1_length += 1;
        } else {
        }
    });

    list2_length = list_length - list1_length - 1;

    list_data[list_length - 1] = list_data[list1_length].cast();
    list_data[list1_length] = pivot.cast();

    arraylist_sort_internal(list_data.cast(), list1_length.cast(), compare_func.cast());

    arraylist_sort_internal((list_data + list1_length + 1).cast(), list2_length.cast(), compare_func.cast());
}
