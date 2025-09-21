pub fn binomial_heap_pop(mut heap: Ptr<BinomialHeap>) -> BinomialHeapValue {
    let mut least_tree: Ptr<BinomialTree> = Default::default();
    let mut fake_heap: BinomialHeap = Default::default();
    let mut result: BinomialHeapValue = Default::default();
    let mut i: u32 = Default::default();
    let mut least_index: u32 = UINT_MAX!();

    if (heap.num_values == 0).as_bool() {
        return BINOMIAL_HEAP_NULL!();
    }

    least_index = UINT_MAX!();

    c_for!(i = 0; i < heap.roots_length; i.prefix_plus_plus(); {
        if (heap.roots[i] == NULL!()).as_bool() {
            continue;
        }

        if (least_index == UINT_MAX!() || 
            binomial_heap_cmp(heap.cast(), heap.roots[i].value.cast(), heap.roots[least_index].value.cast()) < 0).as_bool() {
            least_index = i;
        }
    });

    least_tree = heap.roots[least_index].cast();
    heap.roots[least_index] = NULL!();

    fake_heap.heap_type = heap.heap_type.cast();
    fake_heap.compare_func = heap.compare_func.cast();
    fake_heap.roots = least_tree.subtrees.cast();
    fake_heap.roots_length = least_tree.order.cast();

    if binomial_heap_merge(heap.cast(), c_ref!(fake_heap).cast()).as_bool() {
        result = least_tree.value.cast();
        binomial_tree_unref(least_tree.cast());

        heap.num_values -= 1;

        return result.cast();
    } else {
        heap.roots[least_index] = least_tree.cast();

        return BINOMIAL_HEAP_NULL!();
    }
}
