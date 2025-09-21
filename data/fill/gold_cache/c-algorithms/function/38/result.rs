pub fn binomial_heap_cmp(mut heap: Ptr<BinomialHeap>, mut data1: BinomialHeapValue, mut data2: BinomialHeapValue) -> i32 {
    if (heap.heap_type == BINOMIAL_HEAP_TYPE_MIN!()).as_bool() {
        return (heap.compare_func)(data1.cast(), data2.cast()).cast();
    } else {
        return (-(heap.compare_func)(data1.cast(), data2.cast())).cast();
    }
}
