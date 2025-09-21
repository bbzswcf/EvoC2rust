pub fn binomial_heap_merge_undo(mut new_roots: Ptr<Ptr<BinomialTree>>, mut count: u32) {
    let mut i: u32 = Default::default();

    c_for!(let mut i = 0; i <= count; i.prefix_plus_plus(); {
        binomial_tree_unref(new_roots[i].cast());
    });

    c_free!(new_roots);
}
