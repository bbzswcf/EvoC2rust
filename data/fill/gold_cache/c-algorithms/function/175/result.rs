pub fn list_to_array(mut list: Ptr<ListEntry>) -> Ptr<ListValue> {
    let mut rover: Ptr<ListEntry> = Default::default();
    let mut array: Ptr<ListValue> = Default::default();
    let mut length: u32 = Default::default();
    let mut i: u32 = Default::default();

    length = list_length(list.cast()).cast();

    array = c_malloc!(c_sizeof!(ListValue) * length);

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
