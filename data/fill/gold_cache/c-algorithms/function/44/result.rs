pub fn binomial_heap_new(mut heap_type: BinomialHeapType, mut compare_func: BinomialHeapCompareFunc) -> Ptr<BinomialHeap> {
    let mut new_heap: Ptr<BinomialHeap> = c_calloc!(1, c_sizeof!(BinomialHeap));

    if (new_heap == NULL!()).as_bool() {
        return NULL!();
    }

    new_heap.heap_type = heap_type.cast();
    new_heap.compare_func = compare_func.cast();

    return new_heap.cast();
}
