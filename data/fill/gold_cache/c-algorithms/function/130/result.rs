pub fn set_free_entry(mut set: Ptr<Set>, mut entry: Ptr<SetEntry>) {
    if (set.free_func != NULL!()).as_bool() {
        (set.free_func)(entry.data.cast());
    }
    c_free!(entry);
}
