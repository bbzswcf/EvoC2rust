pub fn pointer_equal(mut location1: Ptr<Void>, mut location2: Ptr<Void>) -> i32 {
    return (location1 == location2).cast();
}
