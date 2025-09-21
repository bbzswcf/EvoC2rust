pub fn set_query(mut set: Ptr<Set>, mut data: SetValue) -> i32 {
    let mut rover: Ptr<SetEntry> = Default::default();
    let mut index: u32 = Default::default();

    index = (set.hash_func)(data.cast()) % set.table_size;

    rover = set.table[index].cast();

    while (rover != NULL!()).as_bool() {
        if ((set.equal_func)(data.cast(), rover.data.cast()) != 0).as_bool() {
            return 1;
        }

        rover = rover.next.cast();
    }

    return 0;
}
