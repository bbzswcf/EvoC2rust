pub fn binary_heap_new(mut heap_type: BinaryHeapType, mut compare_func: BinaryHeapCompareFunc) -> Ptr<BinaryHeap> {
    let mut heap: Ptr<BinaryHeap> = c_malloc!(c_sizeof!(BinaryHeap));

    if (heap == NULL!()).as_bool() {
        return NULL!();
    }

    heap.heap_type = heap_type.cast();
    heap.num_values = 0;
    heap.compare_func = compare_func.cast();

    heap.alloced_size = 16;
    heap.values = c_malloc!(c_sizeof!(BinaryHeapValue) * heap.alloced_size);

    if (heap.values == NULL!()).as_bool() {
        c_free!(heap);
        return NULL!();
    }

    return heap.cast();
}
