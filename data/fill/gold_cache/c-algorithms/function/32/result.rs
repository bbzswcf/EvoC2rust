pub fn arraylist_remove(mut arraylist: Ptr<ArrayList>, mut index: u32) {
    arraylist_remove_range(arraylist.cast(), index.cast(), 1);
}
