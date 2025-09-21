pub fn arraylist_prepend(mut arraylist: Ptr<ArrayList>, mut data: ArrayListValue) -> i32 {
    return arraylist_insert(arraylist.cast(), 0, data.cast()).cast();
}
