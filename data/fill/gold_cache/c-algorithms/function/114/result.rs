pub fn string_nocase_hash(mut string: Ptr<Void>) -> u32 {
    let mut result: u32 = 5381;
    let mut p: Ptr<u8> = string.cast::<Ptr<u8>>();

    while (*p != b'\0').as_bool() {
        result = (result << 5) + result + c_tolower!(*p).cast::<u32>();
        p += 1;
    }

    return result.cast();
}
