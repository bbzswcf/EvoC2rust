pub fn binary_heap_pop(mut heap: Ptr<BinaryHeap>) -> BinaryHeapValue {
    let mut result: BinaryHeapValue = Default::default();
    let mut new_value: BinaryHeapValue = Default::default();
    let mut index: u32 = Default::default();
    let mut next_index: u32 = Default::default();
    let mut child1: u32 = Default::default();
    let mut child2: u32 = Default::default();

    if (heap.num_values == 0) {
        return BINARY_HEAP_NULL!();
    }

    result = heap.values[0];

    let tmp0 = heap.num_values - 1;
    new_value = heap.values[tmp0];
    heap.num_values.suffix_minus_minus();

    index = 0;

    loop {
        child1 = index * 2 + 1;
        child2 = index * 2 + 2;

        if (child1 < heap.num_values) && (binary_heap_cmp(heap, new_value, heap.values[child1]) > 0) {
            if (child2 < heap.num_values) && (binary_heap_cmp(heap, heap.values[child1], heap.values[child2]) > 0) {
                next_index = child2;
            } else {
                next_index = child1;
            }
        } else if (child2 < heap.num_values) && (binary_heap_cmp(heap, new_value, heap.values[child2]) > 0) {
            next_index = child2;
        } else {
            heap.values[index] = new_value;
            break;
        }

        heap.values[index] = heap.values[next_index];

        index = next_index;
    }

    return result;
}
