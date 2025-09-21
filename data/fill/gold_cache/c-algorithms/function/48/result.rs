pub fn binomial_heap_num_entries(mut heap: Ptr<BinomialHeap>) -> u32 {
    return heap.num_values.cast();
}
