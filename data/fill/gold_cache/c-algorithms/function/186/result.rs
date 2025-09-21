pub fn int_compare(mut vlocation1: Ptr<Void>, mut vlocation2: Ptr<Void>) -> i32 {
    let mut location1: Ptr<i32>;
    let mut location2: Ptr<i32>;

    location1 = vlocation1.cast::<Ptr<i32>>();
    location2 = vlocation2.cast::<Ptr<i32>>();

    if (*location1 < *location2).as_bool() {
        return -1;
    } else if (*location1 > *location2).as_bool() {
        return 1;
    } else {
        return 0;
    }
}
