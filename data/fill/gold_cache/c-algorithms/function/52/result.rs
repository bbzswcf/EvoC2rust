pub fn string_nocase_compare(mut string1: Ptr<Void>, mut string2: Ptr<Void>) -> i32 {
    let mut p1: Ptr<u8>;
    let mut p2: Ptr<u8>;
    let mut c1: i32;
    let mut c2: i32;

    p1 = string1.cast::<Ptr<u8>>();
    p2 = string2.cast::<Ptr<u8>>();

    loop {
        c1 = c_tolower!(*p1).cast();
        c2 = c_tolower!(*p2).cast();

        if (c1 != c2).as_bool() {
            if (c1 < c2).as_bool() {
                return -1;
            } else {
                return 1;
            }
        }

        if (c1 == b'\0' as i32).as_bool() {
            break;
        }

        p1.prefix_plus_plus();
        p2.prefix_plus_plus();
    }

    return 0;
}
