pub fn arraylist_remove_range(mut arraylist: Ptr<ArrayList>, mut index: u32, mut length: u32) {
    if (index > arraylist.length || index + length > arraylist.length) {
        return;
    }
    c_memmove!(
        c_ref!(arraylist.data[index]),
        c_ref!(arraylist.data[index + length]),
        (arraylist.length - (index + length)) * c_sizeof!(ArrayListValue)
    );
    arraylist.length -= length;
}
