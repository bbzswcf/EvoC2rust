pub fn slist_to_array(mut list: Ptr<SListEntry>) -> Ptr<SListValue> {
    let mut rover: Ptr<SListEntry> = Default::default();
    let mut array: Ptr<SListValue> = Default::default();
    let mut length: u32 = Default::default();
    let mut i: u32 = Default::default();

    length = slist_length(list.cast()).cast();

    array = c_malloc!(c_sizeof!(SListValue) * length);

    if (array == NULL!()).as_bool() {
        return NULL!();
    }

    rover = list.cast();

    c_for!(i = 0; i < length; i.prefix_plus_plus(); {
        array[i] = rover.data.cast();

        rover = rover.next.cast();
    });

    return array.cast();
}
