pub fn arraylist_enlarge(mut arraylist: Ptr<ArrayList>) -> i32 {
    let mut data: Ptr<ArrayListValue>;
    let mut newsize: u32;

    newsize = arraylist._alloced * 2;

    data = c_realloc!(arraylist.data, c_sizeof!(ArrayListValue) * newsize);

    if (data == NULL!()).as_bool() {
        return 0;
    } else {
        arraylist.data = data.cast();
        arraylist._alloced = newsize.cast();

        return 1;
    }
}
