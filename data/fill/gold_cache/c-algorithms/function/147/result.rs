pub fn pointer_hash(mut location: Ptr<Void>) -> u32 {
    return location.cast::<u64>().cast::<u32>();
}
