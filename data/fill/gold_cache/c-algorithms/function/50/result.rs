pub fn string_compare(mut string1: Ptr<Void>, mut string2: Ptr<Void>) -> i32 {
    let mut result: i32 = Default::default();

    result = c_strcmp!(string1.cast::<Ptr<u8>>(), string2.cast::<Ptr<u8>>()).cast();

    if (result < 0).as_bool() {
        return -1;
    } else if (result > 0).as_bool() {
        return 1;
    } else {
        return 0;
    }
}
