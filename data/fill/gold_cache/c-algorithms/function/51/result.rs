pub fn string_nocase_equal(mut string1: Ptr<Void>, mut string2: Ptr<Void>) -> i32 {
    return (string_nocase_compare(string1.cast::<Ptr<u8>>(), string2.cast::<Ptr<u8>>()) == 0).cast();
}
