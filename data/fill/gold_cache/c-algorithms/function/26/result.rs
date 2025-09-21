pub fn arraylist_free(mut arraylist: Ptr<ArrayList>) {
    if (arraylist != NULL!()).as_bool() {
        c_free!(arraylist.data);
        c_free!(arraylist);
    }
}
