pub fn binomial_heap_free(mut heap: Ptr<BinomialHeap>) {
    let mut i: u32 = Default::default();

    c_for!(i = 0; i < heap.roots_length; i.prefix_plus_plus(); {
        binomial_tree_unref(heap.roots[i].cast());
    });

    c_free!(heap.roots);
    c_free!(heap);
}
