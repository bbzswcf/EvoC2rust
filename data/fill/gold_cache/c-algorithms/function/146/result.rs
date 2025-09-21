pub fn pointer_compare(mut location1: Ptr<Void>, mut location2: Ptr<Void>) -> i32 {
    if (location1 < location2).as_bool() {
        return -1;
    } else if (location1 > location2).as_bool() {
        return 1;
    } else {
        return 0;
    }
}
