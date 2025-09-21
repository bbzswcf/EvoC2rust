pub fn binomial_heap_insert(mut heap: Ptr<BinomialHeap>, mut value: BinomialHeapValue) -> i32 {
    let mut fake_heap: BinomialHeap = Default::default();
    let mut new_tree: Ptr<BinomialTree>;
    let mut result: i32;

    new_tree = c_malloc!(c_sizeof!(BinomialTree));

    if (new_tree == NULL!()).as_bool() {
        return 0;
    }

    new_tree.value = value.cast();
    new_tree.order = 0;
    new_tree.refcount = 1;
    new_tree.subtrees = NULL!();

    fake_heap.heap_type = heap.heap_type.cast();
    fake_heap.compare_func = heap.compare_func.cast();
    fake_heap.num_values = 1;
    fake_heap.roots = c_ref!(new_tree).cast();
    fake_heap.roots_length = 1;

    result = binomial_heap_merge(heap.cast(), c_ref!(fake_heap).cast()).cast();

    if (result != 0).as_bool() {
        heap.num_values.prefix_plus_plus();
    }

    binomial_tree_unref(new_tree.cast());

    return result.cast();
}
