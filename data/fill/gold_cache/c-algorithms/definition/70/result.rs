#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct _SortedArray {
    pub data: Ptr<SortedArrayValue>,
    pub length: u32,
    pub _alloced: u32,
    pub equ_func: SortedArrayEqualFunc,
    pub cmp_func: SortedArrayCompareFunc,
}
