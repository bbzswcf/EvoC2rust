pub fn binary_heap_insert(mut heap: Ptr<BinaryHeap>, mut value: BinaryHeapValue) -> i32 {
    let mut new_values: Ptr<BinaryHeapValue>;
    let mut index: u32 = Default::default();
    let mut new_size: u32 = Default::default();
    let mut parent: u32 = Default::default();

    if (heap.num_values >= heap.alloced_size).as_bool() {
        new_size = heap.alloced_size * 2;
        new_values = c_realloc!(heap.values, c_sizeof!(BinaryHeapValue) * new_size);

        if (new_values == NULL!()).as_bool() {
            return 0;
        }

        heap.alloced_size = new_size;
        heap.values = new_values;
    }

    index = heap.num_values;
    heap.num_values.prefix_plus_plus();

    while (index > 0).as_bool() {
        parent = (index - 1) / 2;

        if (binary_heap_cmp(heap.cast(), heap.values[parent].cast(), value.cast()) < 0).as_bool() {
            break;
        } else {
            heap.values[index] = heap.values[parent].cast();
            index = parent;
        }
    }

    heap.values[index] = value.cast();
    return 1;
}
