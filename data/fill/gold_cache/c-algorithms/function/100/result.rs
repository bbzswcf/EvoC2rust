pub fn slist_find_data(mut list: Ptr<SListEntry>, mut callback: SListEqualFunc, mut data: SListValue) -> Ptr<SListEntry> {
    let mut rover: Ptr<SListEntry> = Default::default();

    rover = list.cast();
    while (rover != NULL!()).as_bool() {
        if (callback(rover.data.cast(), data.cast()) != 0).as_bool() {
            return rover.cast();
        }
        rover = rover.next.cast();
    }

    return NULL!();
}
