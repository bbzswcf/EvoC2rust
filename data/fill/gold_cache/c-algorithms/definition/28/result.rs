#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct _BinomialHeap {
    pub heap_type: BinomialHeapType,
    pub compare_func: BinomialHeapCompareFunc,
    pub num_values: u32,
    pub roots: Ptr<Ptr<BinomialTree>>,
    pub roots_length: u32,
}
