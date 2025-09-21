pub fn arraylist_append(mut arraylist: Ptr<ArrayList>, mut data: ArrayListValue) -> i32 {
    return arraylist_insert(arraylist.cast(), arraylist.length.cast(), data.cast()).cast();
}
