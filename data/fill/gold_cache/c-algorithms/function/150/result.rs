pub fn binary_heap_free(mut heap: Ptr<BinaryHeap>) {
    c_free!(heap.values);
    c_free!(heap);
}
