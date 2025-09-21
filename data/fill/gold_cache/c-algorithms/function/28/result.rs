pub fn arraylist_insert(mut arraylist: Ptr<ArrayList>, mut index: u32, mut data: ArrayListValue) -> i32 {
    if (index > arraylist.length) {
        return 0;
    }

    if (arraylist.length + 1 > arraylist._alloced) {
        if !arraylist_enlarge(arraylist).as_bool() {
            return 0;
        }
    }

    c_memmove!(
        c_ref!(arraylist.data[index + 1]),
        c_ref!(arraylist.data[index]),
        (arraylist.length - index) * c_sizeof!(ArrayListValue)
    );

    arraylist.data[index] = data;
    arraylist.length.prefix_plus_plus();

    return 1;
}
