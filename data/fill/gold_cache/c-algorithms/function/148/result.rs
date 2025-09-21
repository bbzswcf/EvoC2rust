pub fn binary_heap_cmp(mut heap: Ptr<BinaryHeap>, mut data1: BinaryHeapValue, mut data2: BinaryHeapValue) -> i32 {
    if (heap.heap_type == BINARY_HEAP_TYPE_MIN!()).as_bool() {
        return (heap.compare_func)(data1.cast(), data2.cast()).cast();
    } else {
        return (-(heap.compare_func)(data1.cast(), data2.cast())).cast();
    }
}
