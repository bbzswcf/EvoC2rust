pub fn string_equal(mut string1: Ptr<Void>, mut string2: Ptr<Void>) -> i32 {
    return (c_strcmp!(string1.cast::<Ptr<u8>>(), string2.cast::<Ptr<u8>>()) == 0).cast();
}
