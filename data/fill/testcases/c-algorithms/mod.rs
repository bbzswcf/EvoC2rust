pub mod bloom_filter_h;
pub mod binomial_heap_c;
pub mod list_h;
pub mod sortedarray_c;
pub mod sortedarray_h;
pub mod hash_int_h;
pub mod hash_string_h;
pub mod compare_pointer_h;
pub mod compare_pointer_c;
pub mod set_c;
pub mod rb_tree_h;
pub mod slist_c;
pub mod rb_tree_c;
pub mod set_h;
pub mod slist_h;
pub mod bloom_filter_c;
pub mod avl_tree_c;
pub mod binary_heap_c;
pub mod arraylist_h;
pub mod compare_int_c;
pub mod binomial_heap_h;
pub mod list_c;
pub mod arraylist_c;
pub mod compare_string_c;
pub mod queue_c;
pub mod queue_h;
pub mod avl_tree_h;
pub mod hash_string_c;
pub mod hash_int_c;
pub mod hash_pointer_c;
pub mod compare_string_h;
pub mod hash_table_c;
pub mod hash_pointer_h;
pub mod binary_heap_h;
pub mod trie_h;
pub mod compare_int_h;
pub mod hash_table_h;
pub mod trie_c;


use bloom_filter_h::*;
use binomial_heap_c::*;
use list_h::*;
use sortedarray_c::*;
use sortedarray_h::*;
use hash_int_h::*;
use hash_string_h::*;
use compare_pointer_h::*;
use compare_pointer_c::*;
use set_c::*;
use rb_tree_h::*;
use slist_c::*;
use rb_tree_c::*;
use set_h::*;
use slist_h::*;
use bloom_filter_c::*;
use avl_tree_c::*;
use binary_heap_c::*;
use arraylist_h::*;
use compare_int_c::*;
use binomial_heap_h::*;
use list_c::*;
use arraylist_c::*;
use compare_string_c::*;
use queue_c::*;
use queue_h::*;
use avl_tree_h::*;
use hash_string_c::*;
use hash_int_c::*;
use hash_pointer_c::*;
use compare_string_h::*;
use hash_table_c::*;
use hash_pointer_h::*;
use binary_heap_h::*;
use trie_h::*;
use compare_int_h::*;
use hash_table_h::*;
use trie_c::*;

use crate::translation_utils::*;

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! num_test_values { () => { 200 }; }
    static mut variable1: i32 = 10;
    static mut variable2: i32 = 20;
    static mut variable3: i32 = 30;
    static mut variable4: i32 = 40;
    static mut test_array: Array<i32, 1000> = arr![0; 1000];
    static mut COUNTER: i32 = 0;
    static mut test_array2: Array<i32, 10000> = arr![0; 10000];
    static mut test_array3: Array<i32, 1000> = arr![0; 1000];
    const value1: i32 = 1;
	const value2: i32 = 2;
	const value3: i32 = 3;
	const value4: i32 = 4;
    static mut allocated_keys: i32 = 0;
	static mut allocated_values: i32 = 0;
    static mut vari1: i32 = 50;
    static mut vari2: i32 = 0;
    static mut vari3: i32 = 0;
    static mut vari4: i32 = 0;
    static mut var1: i32 = 0;
    static mut var2: i32 = 0;
    static mut var3: i32 = 0;
    static mut var4: i32 = 0;
    static mut test_array4: Array<i32, 1000> = arr![0; 1000];
    static mut variabl1: i32 = 50;
    static mut variabl2: i32 = 0;
    static mut variabl3: i32 = 0;
    static mut variabl4: i32 = 0;
    macro_rules! test_size { () => { 20 }; }
    macro_rules! test_remove_el { () => { 15 }; }
    macro_rules! test_remove_range { () => { 7 }; }
    macro_rules! test_remove_range_length { () => { 4 }; }
    static mut test_array5: Array<i32, 10000> = arr![0; 10000];
    static mut bin_key: Array<u8, 7> = arr![b'a', b'b', b'c', b'\0', 1, 2, 0xff];
    static mut bin_key2: Array<u8, 8> = arr![b'a', b'b', b'c', b'\0', 1, 2, 0xff, b'\0'];
    static mut bin_key3: Array<u8, 3> = arr![b'a', b'b', b'c'];
    static mut bin_key4: Array<u8, 4> = arr![b'z', b'\0', b'z', b'z'];    
    macro_rules! long_string_len { () => { 4096 } }

    fn generate_trie() -> Ptr<Trie> {
        unsafe {
            let mut trie: Ptr<Trie>;
            let mut i: i32;
            let mut entries: u32;
            let mut buf: Array<u8, 32> = arr![0; 32];
            trie = trie_new();
            entries = 0;
            c_for!(i = 0; i < 10000; i += 1; {
                test_array5[i] = i;
                c_sprintf!(buf, cstr!("{}"), i);
                assert!(trie_insert(trie, buf.cast(), c_ref!(test_array5[i]).cast()) != 0);
                entries += 1;
                assert!(trie_num_entries(trie) == entries);
            });
            return trie;
        }}
    
        #[test]
        fn test_trie_new_free() {
            let mut trie: Ptr<Trie>;
            trie = trie_new();
            assert!(trie != null!());
            trie_free(trie);
            trie = trie_new();
            assert!(trie_insert(trie, cstr!("hello"), cstr!("there")) != 0);
            assert!(trie_insert(trie, cstr!("hell"), cstr!("testing")) != 0);
            assert!(trie_insert(trie, cstr!("testing"), cstr!("testing")) != 0);
            assert!(trie_insert(trie, cstr!(""), cstr!("asfasf")) != 0);
            trie_free(trie);
            trie = trie_new();
            assert!(trie_insert(trie, cstr!("hello"), cstr!("there")) != 0);
            assert!(trie_remove(trie, cstr!("hello")) != 0);
            trie_free(trie);
            test_no_memory_leak!();
        }
    
        #[test]
        fn test_trie_insert() {
        unsafe {
            let mut trie: Ptr<Trie>;
            let mut entries: u32;
            let mut allocated: usize;
            trie = generate_trie();
            entries = trie_num_entries(trie);
            assert!(trie_insert(trie, cstr!("hello world"), null!()) == 0);
            assert!(trie_num_entries(trie) == entries);
            trie_free(trie);
            test_no_memory_leak!();
        }}
    
        #[test]
        fn test_trie_lookup() {
        unsafe {
            let mut trie: Ptr<Trie>;
            let mut buf: Array<u8, 32> = arr![0; 32];
            let mut val: Ptr<i32>;
            let mut i: i32;
            trie = generate_trie();
            assert!(trie_lookup(trie, cstr!("000000000000000")) == TRIE_NULL!());
            assert!(trie_lookup(trie, cstr!("")) == TRIE_NULL!());
            c_for!(i = 0; i < 10000; i += 1; {
                c_sprintf!(buf, cstr!("{}"), i);
                val = trie_lookup(trie, buf.cast()).cast();
                assert!(*val == i);
            });
            trie_free(trie);
            test_no_memory_leak!();
        }}
    
        #[test]
        fn test_trie_remove() {
        unsafe {
            let mut trie: Ptr<Trie>;
            let mut buf: Array<u8, 32> = arr![0; 32];
            let mut i: i32;
            let mut entries: u32;
            trie = generate_trie();
            assert!(trie_remove(trie, cstr!("000000000000000")) == 0);
            assert!(trie_remove(trie, cstr!("")) == 0);
            entries = trie_num_entries(trie);
            assert!(entries == 10000);
            c_for!(i = 0; i < 10000; i += 1; {
                c_sprintf!(buf, cstr!("{}"), i);
                assert!(trie_remove(trie, buf.cast()) != 0);
                entries -= 1;
                assert!(trie_num_entries(trie) == entries);
            });
            trie_free(trie);
            test_no_memory_leak!();
        }}
    
        #[test]
        fn test_trie_replace() {
        unsafe {
            let mut trie: Ptr<Trie>;
            let mut val: Ptr<i32>;
            trie = generate_trie();
            val = c_malloc!(c_sizeof!(i32));
            *val = 999;
            assert!(trie_insert(trie, cstr!("999"), val.cast()) != 0);
            assert!(trie_num_entries(trie) == 10000);
            assert!(trie_lookup(trie, cstr!("999")) == val.cast());
            c_free!(val);
            trie_free(trie);
            test_no_memory_leak!();
        }}
    
        #[test]
        fn test_trie_insert_empty() {
        unsafe {
            let mut trie: Ptr<Trie>;
            let mut buf: CStr = cstr!("");
            trie = trie_new();
            assert!(trie_insert(trie, cstr!(""), buf) != 0);
            assert!(trie_num_entries(trie) != 0);
            assert!(trie_lookup(trie, cstr!("")) == buf);
            assert!(trie_remove(trie, cstr!("")) != 0);
            assert!(trie_num_entries(trie) == 0);
            trie_free(trie);
            test_no_memory_leak!();
        }}
    
        #[test]
        fn test_trie_free_long() {
        unsafe {
            let mut long_string: CStr;
            let mut trie: Ptr<Trie>;
            long_string = c_malloc!(long_string_len!());
            c_memset!(long_string, b'A', long_string_len!());
            long_string[long_string_len!() - 1] = b'\0';
            trie = trie_new();
            trie_insert(trie, long_string, long_string);
            trie_free(trie);
            c_free!(long_string);
            test_no_memory_leak!();
        }}
    
        #[test]
        fn test_trie_negative_keys() {
        unsafe {
            let mut my_key: CStr = CStr::from(&[b'a', b'b', b'c', -50i8 as u8, -20i8 as u8, b'\0']);
            let mut trie: Ptr<Trie>;
            let mut value: CStr;
            trie = trie_new();
            assert!(trie_insert(trie, my_key, cstr!("hello world")) != 0);
            value = trie_lookup(trie, my_key);
            assert!(c_strcmp!(value, cstr!("hello world")) == 0);
            assert!(trie_remove(trie, my_key) != 0);
            assert!(trie_remove(trie, my_key) == 0);
            assert!(trie_lookup(trie, my_key) == null!());
            trie_free(trie);
            test_no_memory_leak!();
        }}
    
        fn generate_binary_trie() -> Ptr<Trie> {
        unsafe {
            let mut trie: Ptr<Trie>;
            trie = trie_new();
            assert!(trie_insert_binary(trie, bin_key2.cast(), bin_key2.len() as i32, cstr!("goodbye world")) != 0);
            assert!(trie_insert_binary(trie, bin_key.cast(), bin_key.len() as i32, cstr!("hello world")) != 0);
            return trie;
        }}
        
        fn test_trie_insert_binary() {
        unsafe {
            let mut trie: Ptr<Trie>;
            let mut value: CStr;
            trie = generate_binary_trie();
            assert!(trie_insert_binary(trie, bin_key.cast(), bin_key.len() as i32, cstr!("hi world")) != 0);
            assert!(trie_insert_binary(trie, bin_key3.cast(), bin_key3.len() as i32, null!()) == 0);
            value = trie_lookup_binary(trie, bin_key.cast(), bin_key.len() as i32);
            assert!(c_strcmp!(value, cstr!("hi world")) == 0);
            value = trie_lookup_binary(trie, bin_key2.cast(), bin_key2.len() as i32);
            assert!(c_strcmp!(value, cstr!("goodbye world")) == 0);
            trie_free(trie);
            test_no_memory_leak!();
        }}
    
        #[test]
        fn test_trie_remove_binary() {
        unsafe {
            let mut trie: Ptr<Trie>;
            let mut value: CStr;
            trie = generate_binary_trie();
            value = trie_lookup_binary(trie, bin_key3.cast(), bin_key3.len() as i32);
            assert!(value == null!());
            assert!(trie_remove_binary(trie, bin_key3.cast(), bin_key3.len() as i32) == 0);
            assert!(trie_lookup_binary(trie, bin_key4.cast(), bin_key4.len() as i32) == null!());
            assert!(trie_remove_binary(trie, bin_key4.cast(), bin_key4.len() as i32) == 0);
            assert!(trie_remove_binary(trie, bin_key2.cast(), bin_key2.len() as i32) != 0);
            assert!(trie_lookup_binary(trie, bin_key2.cast(), bin_key2.len() as i32) == null!());
            assert!(trie_lookup_binary(trie, bin_key.cast(), bin_key.len() as i32) != null!());
            assert!(trie_remove_binary(trie, bin_key.cast(), bin_key.len() as i32) != 0);
            assert!(trie_lookup_binary(trie, bin_key.cast(), bin_key.len() as i32) == null!());
            trie_free(trie);
            test_no_memory_leak!();
        }}

    fn check_sorted_prop(mut sortedarray: Ptr<SortedArray>) {
        let mut i: u32;
        c_for!(i = 1; i < sortedarray_length(sortedarray); i += 1; {
            assert!(int_compare(sortedarray_get(sortedarray, i - 1).cast(), sortedarray_get(sortedarray, i).cast()) <= 0);
        });
    }

    fn free_sorted_ints(mut sortedarray: Ptr<SortedArray>) {
        let mut i: u32;
        c_for!(i = 0; i < sortedarray_length(sortedarray); i += 1; {
            let mut pi: Ptr<i32> = sortedarray_get(sortedarray, i).cast();
            c_free!(pi);
        });
        sortedarray_free(sortedarray);
    }

    fn generate_sortedarray_equ(mut equ_func: SortedArrayEqualFunc) -> Ptr<SortedArray> {
        let mut sortedarray: Ptr<SortedArray>;
        let mut i: u32;
        let array: Array<i32, {test_size!()}> = arr![10, 12, 12, 1, 2, 3, 6, 7, 2, 23, 13, 23, 23, 34, 31, 9, 21, -2, -12, -4];
        sortedarray = sortedarray_new(0, equ_func, func!(int_compare));
        c_for!(i = 0; i < test_size!(); i += 1; {
            let mut pi: Ptr<i32> = c_malloc!(c_sizeof!(i32));
            *pi = array[i];
            sortedarray_insert(sortedarray, pi.cast());
        });
        sortedarray
    }

    fn generate_sortedarray() -> Ptr<SortedArray> {
        generate_sortedarray_equ(func!(int_equal))
    }

    #[test]
    fn test_sortedarray_new_free() {
        let mut sortedarray: Ptr<SortedArray>;
        sortedarray = sortedarray_new(0, func!(int_equal), func!(int_compare));
        assert!(sortedarray != null!());
        sortedarray_free(sortedarray);
        sortedarray_free(null!());
    }

    #[test]
    fn test_sortedarray_insert() {
        let mut sortedarray: Ptr<SortedArray> = generate_sortedarray();
        let mut i: u32;
        c_for!(i = 0; i < 20; i += 1; {
            let mut i: i32 = i as i32;
            let mut pi: Ptr<i32> = c_malloc!(c_sizeof!(i32));
            *pi = i;
            sortedarray_insert(sortedarray, pi.cast());
        });
        check_sorted_prop(sortedarray);
        free_sorted_ints(sortedarray);
    }

    #[test]
    fn test_sortedarray_remove() {
        let mut sortedarray: Ptr<SortedArray> = generate_sortedarray();
        let mut ip: Ptr<i32> = sortedarray_get(sortedarray, test_remove_el!() + 1).cast();
        let mut i: i32 = *ip;
        c_free!(sortedarray_get(sortedarray, test_remove_el!()));
        sortedarray_remove(sortedarray, test_remove_el!());
        assert!(*sortedarray_get(sortedarray, test_remove_el!()) == i.cast());
        check_sorted_prop(sortedarray);
        free_sorted_ints(sortedarray);
    }

    #[test]
    fn test_sortedarray_remove_range() {
        let mut sortedarray: Ptr<SortedArray> = generate_sortedarray();
        let mut new: Array<i32, {test_remove_range_length!()}> = Default::default();
        let mut i: u32;
        c_for!(i = 0; i < test_remove_range_length!(); i += 1; {
            new[i] = (*sortedarray_get(sortedarray, test_remove_range!() + test_remove_range_length!() + i)).cast();
        });
        c_for!(i = 0; i < test_remove_range_length!(); i += 1; {
            c_free!(sortedarray_get(sortedarray, test_remove_range!() + i));
        });
        sortedarray_remove_range(sortedarray, test_remove_range!(), test_remove_range_length!());
        c_for!(i = 0; i < test_remove_range_length!(); i += 1; {
            assert!(*sortedarray_get(sortedarray, test_remove_range!() + i) == new[i].cast());
        });
        check_sorted_prop(sortedarray);
        free_sorted_ints(sortedarray);
    }

    #[test]
    fn test_sortedarray_index_of() {
        let mut sortedarray: Ptr<SortedArray> = generate_sortedarray();
        let mut i: u32;
        c_for!(i = 0; i < test_size!(); i += 1; {
            let mut r: i32 = sortedarray_index_of(sortedarray, sortedarray_get(sortedarray, i).cast());
            assert!(r >= 0);
            assert!(*sortedarray_get(sortedarray, r as u32) == *sortedarray_get(sortedarray, i));
        });
        free_sorted_ints(sortedarray);
    }

    fn ptr_equal(mut v1: SortedArrayValue, mut v2: SortedArrayValue) -> i32 {
        (v1 == v2) as i32
    }

    #[test]
    fn test_sortedarray_index_of_equ_key() {
        let mut sortedarray: Ptr<SortedArray> = generate_sortedarray_equ(func!(ptr_equal));
        let mut i: u32;
        c_for!(i = 0; i < test_size!(); i += 1; {
            let mut r: i32 = sortedarray_index_of(sortedarray, sortedarray_get(sortedarray, i).cast());
            assert!(r >= 0);
            assert!(i == r as u32);
        });
        free_sorted_ints(sortedarray);
    }

    #[test]
    fn test_sortedarray_get() {
        let mut i: u32;
        let mut arr: Ptr<SortedArray> = generate_sortedarray();
        c_for!(i = 0; i < sortedarray_length(arr); i += 1; {
            assert!(sortedarray_get(arr, i) == sortedarray_get(arr, i));
            assert!(*sortedarray_get(arr, i) == *sortedarray_get(arr, i));
        });
        free_sorted_ints(arr);
    }

    fn generate_slist() -> Ptr<SListEntry> {
        unsafe {
            let mut list: Ptr<SListEntry> = null!();
            assert!(slist_append(c_ref!(list), c_ref!(variabl1).cast()) != null!());
            assert!(slist_append(c_ref!(list), c_ref!(variabl2).cast()) != null!());
            assert!(slist_append(c_ref!(list), c_ref!(variabl3).cast()) != null!());
            assert!(slist_append(c_ref!(list), c_ref!(variabl4).cast()) != null!());
            return list;
        }}
    
        #[test]
        fn test_slist_append() {
        unsafe {
            let mut list: Ptr<SListEntry> = null!();
            assert!(slist_append(c_ref!(list), c_ref!(variabl1).cast()) != null!());
            assert!(slist_append(c_ref!(list), c_ref!(variabl2).cast()) != null!());
            assert!(slist_append(c_ref!(list), c_ref!(variabl3).cast()) != null!());
            assert!(slist_append(c_ref!(list), c_ref!(variabl4).cast()) != null!());
            assert!(slist_length(list) == 4);
            assert!(slist_nth_data(list, 0) == c_ref!(variabl1).cast());
            assert!(slist_nth_data(list, 1) == c_ref!(variabl2).cast());
            assert!(slist_nth_data(list, 2) == c_ref!(variabl3).cast());
            assert!(slist_nth_data(list, 3) == c_ref!(variabl4).cast());
            slist_free(list);
            test_no_memory_leak!();
        }}
    
        #[test]
        fn test_slist_prepend() {
        unsafe {
            let mut list: Ptr<SListEntry> = null!();
            assert!(slist_prepend(c_ref!(list), c_ref!(variabl1).cast()) != null!());
            assert!(slist_prepend(c_ref!(list), c_ref!(variabl2).cast()) != null!());
            assert!(slist_prepend(c_ref!(list), c_ref!(variabl3).cast()) != null!());
            assert!(slist_prepend(c_ref!(list), c_ref!(variabl4).cast()) != null!());
            assert!(slist_nth_data(list, 0) == c_ref!(variabl4).cast());
            assert!(slist_nth_data(list, 1) == c_ref!(variabl3).cast());
            assert!(slist_nth_data(list, 2) == c_ref!(variabl2).cast());
            assert!(slist_nth_data(list, 3) == c_ref!(variabl1).cast());
            slist_free(list);
            test_no_memory_leak!();
        }}
    
        #[test]
        fn test_slist_free() {
        unsafe {
            let mut list: Ptr<SListEntry>;
            list = generate_slist();
            slist_free(list);
            slist_free(null!());
            test_no_memory_leak!();
        }}
    
        #[test]
        fn test_slist_next() {
        unsafe {
            let mut list: Ptr<SListEntry>;
            let mut rover: Ptr<SListEntry>;
            list = generate_slist();
            rover = list;
            assert!(slist_data(rover) == c_ref!(variabl1).cast());
            rover = slist_next(rover);
            assert!(slist_data(rover) == c_ref!(variabl2).cast());
            rover = slist_next(rover);
            assert!(slist_data(rover) == c_ref!(variabl3).cast());
            rover = slist_next(rover);
            assert!(slist_data(rover) == c_ref!(variabl4).cast());
            rover = slist_next(rover);
            assert!(rover == null!());
            slist_free(list);
            test_no_memory_leak!();
        }}
    
        #[test]
        fn test_slist_nth_entry() {
        unsafe {
            let mut list: Ptr<SListEntry>;
            let mut entry: Ptr<SListEntry>;
            list = generate_slist();
            entry = slist_nth_entry(list, 0);
            assert!(slist_data(entry) == c_ref!(variabl1).cast());
            entry = slist_nth_entry(list, 1);
            assert!(slist_data(entry) == c_ref!(variabl2).cast());
            entry = slist_nth_entry(list, 2);
            assert!(slist_data(entry) == c_ref!(variabl3).cast());
            entry = slist_nth_entry(list, 3);
            assert!(slist_data(entry) == c_ref!(variabl4).cast());
            entry = slist_nth_entry(list, 4);
            assert!(entry == null!());
            entry = slist_nth_entry(list, 400);
            assert!(entry == null!());
            slist_free(list);
            test_no_memory_leak!();
        }}
    
        #[test]
        fn test_slist_nth_data() {
        unsafe {
            let mut list: Ptr<SListEntry>;
            list = generate_slist();
            assert!(slist_nth_data(list, 0) == c_ref!(variabl1).cast());
            assert!(slist_nth_data(list, 1) == c_ref!(variabl2).cast());
            assert!(slist_nth_data(list, 2) == c_ref!(variabl3).cast());
            assert!(slist_nth_data(list, 3) == c_ref!(variabl4).cast());
            assert!(slist_nth_data(list, 4) == null!());
            assert!(slist_nth_data(list, 400) == null!());
            slist_free(list);
            test_no_memory_leak!();
        }}
    
        #[test]
        fn test_slist_length() {
        unsafe {
            let mut list: Ptr<SListEntry>;
            list = generate_slist();
            assert!(slist_length(list) == 4);
            slist_prepend(c_ref!(list), c_ref!(variabl1).cast());
            assert!(slist_length(list) == 5);
            assert!(slist_length(null!()) == 0);
            slist_free(list);
            test_no_memory_leak!();
        }}
    
        #[test]
        fn test_slist_remove_entry() {
        unsafe {
            let mut empty_list: Ptr<SListEntry> = null!();
            let mut list: Ptr<SListEntry>;
            let mut entry: Ptr<SListEntry>;
            list = generate_slist();
            entry = slist_nth_entry(list, 2);
            assert!(slist_remove_entry(c_ref!(list), entry) != 0);
            assert!(slist_length(list) == 3);
            entry = slist_nth_entry(list, 0);
            assert!(slist_remove_entry(c_ref!(list), entry) != 0);
            assert!(slist_length(list) == 2);
            assert!(slist_remove_entry(c_ref!(list), entry) == 0);
            assert!(slist_remove_entry(c_ref!(list), null!()) == 0);
            assert!(slist_remove_entry(c_ref!(empty_list), null!()) == 0);
            slist_free(list);
            test_no_memory_leak!();
        }}
    
        #[test]
        fn test_slist_remove_data() {
        unsafe {
            let mut entries: Array<i32, 13> = arr![89, 4, 23, 42, 4, 16, 15, 4, 8, 99, 50, 30, 4];
            let num_entries: u32 = entries.len() as u32;
            let mut val: i32;
            let mut list: Ptr<SListEntry>;
            let mut i: u32;
            list = null!();
            c_for!(i = 0; i < num_entries; i += 1; {
                slist_prepend(c_ref!(list), c_ref!(entries[i]).cast());
            });
            val = 0;
            assert!(slist_remove_data(c_ref!(list), func!(int_equal), c_ref!(val).cast()) == 0);
            val = 56;
            assert!(slist_remove_data(c_ref!(list), func!(int_equal), c_ref!(val).cast()) == 0);
            val = 8;
            assert!(slist_remove_data(c_ref!(list), func!(int_equal), c_ref!(val).cast()) == 1);
            assert!(slist_length(list) == num_entries - 1);
            val = 4;
            assert!(slist_remove_data(c_ref!(list), func!(int_equal), c_ref!(val).cast()) == 4);
            assert!(slist_length(list) == num_entries - 5);
            val = 89;
            assert!(slist_remove_data(c_ref!(list), func!(int_equal), c_ref!(val).cast()) == 1);
            assert!(slist_length(list) == num_entries - 6);
            slist_free(list);
            test_no_memory_leak!();
        }}
    
        #[test]
        fn test_slist_sort() {
        unsafe {
            let mut list: Ptr<SListEntry>;
            let mut entries: Array<i32, 13> = arr![89, 4, 23, 42, 4, 16, 15, 4, 8, 99, 50, 30, 4];
            let sorted: Array<i32, 13> = arr![4, 4, 4, 4, 8, 15, 16, 23, 30, 42, 50, 89, 99];
            let num_entries: u32 = entries.len() as u32;
            let mut i: u32;
            list = null!();
            c_for!(i = 0; i < num_entries; i += 1; {
                slist_prepend(c_ref!(list), c_ref!(entries[i]).cast());
            });
            slist_sort(c_ref!(list), func!(int_compare));
            assert!(slist_length(list) == num_entries);
            c_for!(i = 0; i < num_entries; i += 1; {
                let mut value: Ptr<i32>;
                value = slist_nth_data(list, i).cast();
                assert!(*value == sorted[i]);
            });
            slist_free(list);
            list = null!();
            slist_sort(c_ref!(list), func!(int_compare));
            assert!(list == null!());
            test_no_memory_leak!();
        }}
    
        #[test]
        fn test_slist_find_data() {
        unsafe {
            let mut entries: Array<i32, 10> = arr![89, 23, 42, 16, 15, 4, 8, 99, 50, 30];
            let num_entries: u32 = entries.len() as u32;
            let mut list: Ptr<SListEntry>;
            let mut result: Ptr<SListEntry>;
            let mut i: u32;
            let mut val: i32;
            let mut data: Ptr<i32>;
            list = null!();
            c_for!(i = 0; i < num_entries; i += 1; {
                slist_append(c_ref!(list), c_ref!(entries[i]).cast());
            });
            c_for!(i = 0; i < num_entries; i += 1; {
                val = entries[i];
                result = slist_find_data(list, func!(int_equal), c_ref!(val).cast());
                assert!(result != null!());
                data = slist_data(result).cast();
                assert!(*data == val);
            });
            val = 0;
            assert!(slist_find_data(list, func!(int_equal), c_ref!(val).cast()) == null!());
            val = 56;
            assert!(slist_find_data(list, func!(int_equal), c_ref!(val).cast()) == null!());
            slist_free(list);
            test_no_memory_leak!();
        }}
    
        #[test]
        fn test_slist_to_array() {
        unsafe {
            let mut list: Ptr<SListEntry>;
            let mut array: Ptr<SListValue>;
            list = generate_slist();
            array = slist_to_array(list);
            assert!(array[0] == c_ref!(variabl1).cast());
            assert!(array[1] == c_ref!(variabl2).cast());
            assert!(array[2] == c_ref!(variabl3).cast());
            assert!(array[3] == c_ref!(variabl4).cast());
            c_free!(array);
            slist_free(list);
            test_no_memory_leak!();
        }}
    
        #[test]
        fn test_slist_iterate() {
        unsafe {
            let mut list: Ptr<SListEntry>;
            let mut iter: SListIterator = Default::default();
            let mut data: Ptr<i32>;
            let mut a: i32 = 0;
            let mut i: i32;
            let mut counter: i32;
            list = null!();
            c_for!(i = 0; i < 50; i += 1; {
                slist_prepend(c_ref!(list), c_ref!(a).cast());
            });
            counter = 0;
            slist_iterate(c_ref!(list), c_ref!(iter));
            slist_iter_remove(c_ref!(iter));
            while slist_iter_has_more(c_ref!(iter)) != 0 {
                data = slist_iter_next(c_ref!(iter)).cast();
                counter += 1;
                if counter % 2 == 0 {
                    slist_iter_remove(c_ref!(iter));
                    slist_iter_remove(c_ref!(iter));
                }
            }
            assert!(slist_iter_next(c_ref!(iter)) == SLIST_NULL!());
            slist_iter_remove(c_ref!(iter));
            assert!(counter == 50);
            assert!(slist_length(list) == 25);
            slist_free(list);
            list = null!();
            counter = 0;
            slist_iterate(c_ref!(list), c_ref!(iter));
            while slist_iter_has_more(c_ref!(iter)) != 0 {
                data = slist_iter_next(c_ref!(iter)).cast();
                counter += 1;
                if counter % 2 == 0 {
                    slist_iter_remove(c_ref!(iter));
                }
            }
            assert!(counter == 0);
            test_no_memory_leak!();
        }}
    
        #[test]
        fn test_slist_iterate_bad_remove() {
        unsafe {
            let mut list: Ptr<SListEntry>;
            let mut iter: SListIterator = Default::default();
            let mut values: Array<i32, 49> = arr![0; 49];
            let mut i: i32;
            let mut val: Ptr<i32>;
            list = null!();
            c_for!(i = 0; i < 49; i += 1; {
                values[i] = i;
                assert!(slist_prepend(c_ref!(list), c_ref!(values[i]).cast()) != null!());
            });
            slist_iterate(c_ref!(list), c_ref!(iter));
            while slist_iter_has_more(c_ref!(iter)) != 0 {
                val = slist_iter_next(c_ref!(iter)).cast();
                if *val % 2 == 0 {
                    assert!(slist_remove_data(c_ref!(list), func!(int_equal), val.cast()) != 0);
                    slist_iter_remove(c_ref!(iter));
                }
            }
            slist_free(list);
            test_no_memory_leak!();
        }}

    fn generate_set() -> Ptr<Set> {
		let mut set: Ptr<Set>;
		let mut buf: Array<u8, 32> = arr![0; 32];
		let mut i: u32;
		let mut value: CStr;
		set = set_new(func!(string_hash), func!(string_equal));
		c_for!(i = 0; i < 10000; i += 1; {
			c_sprintf!(buf, cstr!("{}"), i);
			value = c_strdup!(buf);
			set_insert(set, value);
			assert!(set_num_entries(set) == i + 1);
		});
		set_register_free_function(set, c_free!());
		return set;
	}

    #[test]
	fn test_set_new_free() {
		let mut set: Ptr<Set>;
		let mut i: i32;
		let mut value: Ptr<i32>;
		set = set_new(func!(int_hash), func!(int_equal));
		set_register_free_function(set, c_free!());
		assert!(set != null!());
		c_for!(i = 0; i < 10000; i += 1; {
			value = c_malloc!(c_sizeof!(i32));
			*value = i;
			set_insert(set, value.cast());
		});
		set_free(set);
		test_no_memory_leak!();
	}

    #[test]
	fn test_set_insert() {
		let mut set: Ptr<Set>;
		let mut numbers1 = arr![1, 2, 3, 4, 5, 6];
		let mut numbers2 = arr![5, 6, 7, 8, 9, 10];
		let mut i: i32;
		set = set_new(func!(int_hash), func!(int_equal));
		c_for!(i = 0; i < 6; i += 1; {
			set_insert(set, c_ref!(numbers1[i]).cast());
		});
		c_for!(i = 0; i < 6; i += 1; {
			set_insert(set, c_ref!(numbers2[i]).cast());
		});
		assert!(set_num_entries(set) == 10);
		set_free(set);
		test_no_memory_leak!();
	}

	#[test]
	fn test_set_query() {
		let mut set: Ptr<Set>;
		let mut buf: Array<u8, 32> = arr![0; 32];
		let mut i: i32;
		set = generate_set();
		c_for!(i = 0; i < 10000; i += 1; {
			c_sprintf!(buf, cstr!("{}"), i);
			assert!(set_query(set, buf.cast()) != 0);
		});
		assert!(set_query(set, cstr!("-1")) == 0);
		assert!(set_query(set, cstr!("100001")) == 0);
		set_free(set);
		test_no_memory_leak!();
	}

	#[test]
	fn test_set_remove() {
		let mut set: Ptr<Set>;
		let mut buf: Array<u8, 32> = arr![0; 32];
		let mut i: i32;
		let mut num_entries: u32;
		set = generate_set();
		num_entries = set_num_entries(set);
		assert!(num_entries == 10000);
		c_for!(i = 4000; i < 6000; i += 1; {
			c_sprintf!(buf, cstr!("{}"), i);
			assert!(set_query(set, buf.cast()) != 0);
			assert!(set_remove(set, buf.cast()) != 0);
			assert!(set_num_entries(set) == num_entries - 1);
			assert!(set_query(set, buf.cast()) == 0);
			num_entries -= 1;
		});
		c_for!(i = -1000; i < -500; i += 1; {
			c_sprintf!(buf, cstr!("{}"), i);
			assert!(set_remove(set, buf.cast()) == 0);
			assert!(set_num_entries(set) == num_entries);
		});
		c_for!(i = 50000; i < 51000; i += 1; {
			c_sprintf!(buf, cstr!("{}"), i);
			assert!(set_remove(set, buf.cast()) == 0);
			assert!(set_num_entries(set) == num_entries);
		});
		set_free(set);
		test_no_memory_leak!();
	}

	#[test]
	fn test_set_union() {
        let mut all_values = arr![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
		let mut i: i32;
		let mut set1: Ptr<Set>;
		let mut set2: Ptr<Set>;
		let mut result_set: Ptr<Set>;
		let mut allocated: usize;
		set1 = set_new(func!(int_hash), func!(int_equal));
		c_for!(i = 0; i < 7; i += 1; {
			set_insert(set1, c_ref!(all_values[i]));
		});
		set2 = set_new(func!(int_hash), func!(int_equal));
		c_for!(i = 4; i < 11; i += 1; {
			set_insert(set2, c_ref!(all_values[i]));
		});
		result_set = set_union(set1, set2);
		assert!(set_num_entries(result_set) == 11);
		c_for!(i = 0; i < 11; i += 1; {
			assert!(set_query(result_set, c_ref!(all_values[i])) != 0);
		});
		set_free(result_set);
		set_free(set1);
		set_free(set2);
		test_no_memory_leak!();
	}

	#[test]
	fn test_set_intersection() {
        let mut all_values = arr![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
		let mut i: i32;
		let mut set1: Ptr<Set>;
		let mut set2: Ptr<Set>;
		let mut result_set: Ptr<Set>;
		let mut allocated: usize;
		set1 = set_new(func!(int_hash), func!(int_equal));
		c_for!(i = 0; i < 7; i += 1; {
			set_insert(set1, c_ref!(all_values[i]));
		});
		set2 = set_new(func!(int_hash), func!(int_equal));
		c_for!(i = 4; i < 11; i += 1; {
			set_insert(set2, c_ref!(all_values[i]));
		});
		result_set = set_intersection(set1, set2);
		assert!(set_num_entries(result_set) == 3);
		c_for!(i = 4; i < 7; i += 1; {
			assert!(set_query(result_set, c_ref!(all_values[i])) != 0);
		});
		set_free(set1);
		set_free(set2);
		set_free(result_set);
		test_no_memory_leak!();
	}

	#[test]
	fn test_set_to_array() {
		let mut set: Ptr<Set>;
		let mut values = arr![1; 100];
        let mut array: Ptr<SetValue>;
		let mut i: i32;
		set = set_new(func!(pointer_hash), func!(pointer_equal));
		c_for!(i = 0; i < 100; i += 1; {
			set_insert(set, c_ref!(values[i]).cast());
		});
		array = set_to_array(set);
		c_for!(i = 0; i < 100; i += 1; {
			assert!(*array[i] == 1);
			*array[i] = 0;
		});
		c_free!(array);
		set_free(set);
		test_no_memory_leak!();
	}

	#[test]
	fn test_set_iterating() {
		let mut set: Ptr<Set>;
		let mut iterator: SetIterator = Default::default();
		let mut count: i32;
		set = generate_set();
		count = 0;
		set_iterate(set, c_ref!(iterator));
		while set_iter_has_more(c_ref!(iterator)).as_bool() {
			set_iter_next(c_ref!(iterator));
			count += 1;
		}
		assert!(set_iter_next(c_ref!(iterator)) == SET_NULL!());
		assert!(count == 10000);
		set_free(set);
		let mut set: Ptr<Set>;
		let mut iterator: SetIterator = Default::default();
		set = set_new(func!(int_hash), func!(int_equal));
		set_iterate(set, c_ref!(iterator));
		assert!(set_iter_has_more(c_ref!(iterator)) == 0);
		set_free(set);
		test_no_memory_leak!();
	}

	#[test]
	fn test_set_iterating_remove() {
		let mut set: Ptr<Set>;
		let mut iterator: SetIterator = Default::default();
		let mut count: i32;
		let mut removed: u32;
		let mut value: CStr;
		set = generate_set();
		count = 0;
		removed = 0;
		set_iterate(set, c_ref!(iterator));
		while set_iter_has_more(c_ref!(iterator)).as_bool() {
			value = set_iter_next(c_ref!(iterator));
			if (c_atoi!(value) % 100) == 0 {
				set_remove(set, value);
				removed += 1;
			}
			count += 1;
		}
		assert!(count == 10000);
		assert!(removed == 100);
		assert!(set_num_entries(set) == 10000 - removed);
		set_free(set);
		test_no_memory_leak!();
	}

	fn new_value(mut value: i32) -> Ptr<i32> {
	unsafe { 
		let mut result: Ptr<i32>;
		result = c_malloc!(c_sizeof!(i32));
		*result = value;
		allocated_values += 1; 		
		return result
	}}

	fn free_value(mut value: VoidPtr) {
	unsafe {
		c_free!(value);
		allocated_values -= 1;
	}}

	#[test]
	fn test_set_free_function() {
	unsafe {
		let mut set: Ptr<Set>;
		let mut i: i32;
		let mut value: Ptr<i32>;
		set = set_new(func!(int_hash), func!(int_equal));
		set_register_free_function(set, func!(free_value));
		allocated_values = 0;
		c_for!(i = 0; i < 1000; i += 1; {
			value = new_value(i);
			set_insert(set, value.cast());
		});
		assert!(allocated_values == 1000);
		i = 500;
		set_remove(set, c_ref!(i).cast());
		assert!(allocated_values == 999);
		set_free(set);
		assert!(allocated_values == 0);
		test_no_memory_leak!();
	}}

    fn find_subtree_height(mut node: Ptr<RBTreeNode>) -> i32 {
        let mut left_subtree: Ptr<RBTreeNode>;
        let mut right_subtree: Ptr<RBTreeNode>;
        let mut left_height: i32;
        let mut right_height: i32;
        if node == null!() {
            return 0;
        }
        left_subtree = rb_tree_node_child(node, RB_TREE_NODE_LEFT!());
        right_subtree = rb_tree_node_child(node, RB_TREE_NODE_RIGHT!());
        left_height = find_subtree_height(left_subtree);
        right_height = find_subtree_height(right_subtree);

        if left_height > right_height {
            return left_height + 1;
        } else {
            return right_height + 1;
        }
    }

    fn validate_tree(mut tree: Ptr<RBTree>) {
    }

    fn create_tree() -> Ptr<RBTree> {
        unsafe {
            let mut tree: Ptr<RBTree>;
            let mut i: i32;
            tree = rb_tree_new(func!(int_compare));
            c_for!(i = 0; i < 1000; i += 1; {
                test_array4[i] = i;
                rb_tree_insert(tree, c_ref!(test_array4[i]).cast(), c_ref!(test_array4[i]).cast());
            });
    
            return tree;
        }
    }

    #[test]
    fn test_rb_tree_new() {
        let mut tree: Ptr<RBTree>;
        tree = rb_tree_new(func!(int_compare));
        assert!(tree != null!());
        assert!(rb_tree_root_node(tree) == null!());
        assert!(rb_tree_num_entries(tree) == 0);
        rb_tree_free(tree);
        test_no_memory_leak!();
    }

    #[test]
    fn test_rb_tree_insert_lookup() {
    unsafe {
        let mut tree: Ptr<RBTree>;
        let mut node: Ptr<RBTreeNode>;
        let mut i: i32;
        let mut value: Ptr<i32>;
        tree = rb_tree_new(func!(int_compare));
        c_for!(i = 0; i < 1000; i += 1; {
            test_array4[i] = i;
            rb_tree_insert(tree, c_ref!(test_array4[i]).cast(), c_ref!(test_array4[i]).cast());

            assert!(rb_tree_num_entries(tree) == i + 1);
            validate_tree(tree);
        });
        assert!(rb_tree_root_node(tree) != null!());
        c_for!(i = 0; i < 1000; i += 1; {
            node = rb_tree_lookup_node(tree, c_ref!(i).cast());
            assert!(node != null!());
            value = rb_tree_node_key(node).cast();
            assert!(*value == i);
            value = rb_tree_node_value(node).cast();
            assert!(*value == i);
        });
        i = -1;
        assert!(rb_tree_lookup_node(tree, c_ref!(i).cast()) == null!());
        i = 1000 + 100;
        assert!(rb_tree_lookup_node(tree, c_ref!(i).cast()) == null!());
        rb_tree_free(tree);
        test_no_memory_leak!();
    }}

    #[test]
    fn test_rb_tree_child() {
        let mut tree: Ptr<RBTree>;
        let mut root: Ptr<RBTreeNode>;
        let mut left: Ptr<RBTreeNode>;
        let mut right: Ptr<RBTreeNode>;
        let mut values: Array<i32, 3> = arr![1, 2, 3];
        let mut p: Ptr<i32>;
        let mut i: i32;
        tree = rb_tree_new(func!(int_compare));
        c_for!(i = 0; i < 3; i += 1; {
            rb_tree_insert(tree, c_ref!(values[i]).cast(), c_ref!(values[i]).cast());
        });
        root = rb_tree_root_node(tree);
        p = rb_tree_node_value(root).cast();
        assert!(*p == 2);
        left = rb_tree_node_child(root, RB_TREE_NODE_LEFT!());
        p = rb_tree_node_value(left).cast();
        assert!(*p == 1);
        right = rb_tree_node_child(root, RB_TREE_NODE_RIGHT!());
        p = rb_tree_node_value(right).cast();
        assert!(*p == 3);
        assert!(rb_tree_node_child(root, 10000) == null!());
        assert!(rb_tree_node_child(root, 2) == null!());
        rb_tree_free(tree);
        test_no_memory_leak!();
    }

    #[test]
    fn test_rb_tree_free() {
        let mut tree: Ptr<RBTree>;
        tree = rb_tree_new(func!(int_compare));
        rb_tree_free(tree);
        tree = create_tree();
        rb_tree_free(tree);
        test_no_memory_leak!();
    }

    #[test]
    fn test_rb_tree_lookup() {
    unsafe {
        let mut tree: Ptr<RBTree>;
        let mut i: i32;
        let mut value: Ptr<i32>;
        tree = create_tree();
        c_for!(i = 0; i < 1000; i += 1; {
            value = rb_tree_lookup(tree, c_ref!(i).cast()).cast();
            assert!(value != null!());
            assert!(*value == i);
        });
        i = -1;
        assert!(rb_tree_lookup(tree, c_ref!(i).cast()) == null!());
        i = 1000 + 1;
        assert!(rb_tree_lookup(tree, c_ref!(i).cast()) == null!());
        i = 8724897;
        assert!(rb_tree_lookup(tree, c_ref!(i).cast()) == null!());
        rb_tree_free(tree);
        test_no_memory_leak!();
    }}

    fn generate_queue() -> Ptr<Queue> {
        unsafe {
            let mut queue: Ptr<Queue> = queue_new();
            let mut i: i32;
            c_for!(i = 0; i < 1000; i += 1; {
                queue_push_head(queue, c_ref!(var1).cast());
                queue_push_head(queue, c_ref!(var2).cast());
                queue_push_head(queue, c_ref!(var3).cast());
                queue_push_head(queue, c_ref!(var4).cast());
            });
            return queue;
        }}
    
        #[test]
        fn test_queue_new_free() {
        unsafe {
            let mut i: i32;
            let mut queue: Ptr<Queue>;
            queue = queue_new();
            queue_free(queue);
            queue = queue_new();
            c_for!(i = 0; i < 1000; i += 1; {
                queue_push_head(queue, c_ref!(var1).cast());
            });
            queue_free(queue);
            test_no_memory_leak!();
        }}
    
        #[test]
        fn test_queue_push_head() {
        unsafe {
            let mut queue: Ptr<Queue>;
            let mut i: i32;
            queue = queue_new();
            c_for!(i = 0; i < 1000; i += 1; {
                queue_push_head(queue, c_ref!(var1).cast());
                queue_push_head(queue, c_ref!(var2).cast());
                queue_push_head(queue, c_ref!(var3).cast());
                queue_push_head(queue, c_ref!(var4).cast());
            });
            assert!(!queue_is_empty(queue).as_bool());
            assert!(queue_pop_tail(queue) == c_ref!(var1).cast());
            assert!(queue_pop_tail(queue) == c_ref!(var2).cast());
            assert!(queue_pop_tail(queue) == c_ref!(var3).cast());
            assert!(queue_pop_tail(queue) == c_ref!(var4).cast());
            assert!(queue_pop_head(queue) == c_ref!(var4).cast());
            assert!(queue_pop_head(queue) == c_ref!(var3).cast());
            assert!(queue_pop_head(queue) == c_ref!(var2).cast());
            assert!(queue_pop_head(queue) == c_ref!(var1).cast());
            queue_free(queue);
            test_no_memory_leak!();
        }}
    
        #[test]
        fn test_queue_pop_head() {
        unsafe {
            let mut queue: Ptr<Queue>;
            queue = queue_new();
            assert!(queue_pop_head(queue) == null!());
            queue_free(queue);
            queue = generate_queue();
            while !queue_is_empty(queue).as_bool() {
                assert!(queue_pop_head(queue) == c_ref!(var4).cast());
                assert!(queue_pop_head(queue) == c_ref!(var3).cast());
                assert!(queue_pop_head(queue) == c_ref!(var2).cast());
                assert!(queue_pop_head(queue) == c_ref!(var1).cast());
            }
            assert!(queue_pop_head(queue) == null!());
            queue_free(queue);
            test_no_memory_leak!();
        }}
    
        #[test]
        fn test_queue_peek_head() {
        unsafe {
            let mut queue: Ptr<Queue>;
            queue = queue_new();
            assert!(queue_peek_head(queue) == null!());
            queue_free(queue);
            queue = generate_queue();
            while !queue_is_empty(queue).as_bool() {
                assert!(queue_peek_head(queue) == c_ref!(var4).cast());
                assert!(queue_pop_head(queue) == c_ref!(var4).cast());
                assert!(queue_peek_head(queue) == c_ref!(var3).cast());
                assert!(queue_pop_head(queue) == c_ref!(var3).cast());
                assert!(queue_peek_head(queue) == c_ref!(var2).cast());
                assert!(queue_pop_head(queue) == c_ref!(var2).cast());
                assert!(queue_peek_head(queue) == c_ref!(var1).cast());
                assert!(queue_pop_head(queue) == c_ref!(var1).cast());
            }
            assert!(queue_peek_head(queue) == null!());
            queue_free(queue);
            test_no_memory_leak!();
        }}
    
        #[test]
        fn test_queue_push_tail() {
        unsafe {
            let mut queue: Ptr<Queue>;
            let mut i: i32;
            queue = queue_new();
            c_for!(i = 0; i < 1000; i += 1; {
                queue_push_tail(queue, c_ref!(var1).cast());
                queue_push_tail(queue, c_ref!(var2).cast());
                queue_push_tail(queue, c_ref!(var3).cast());
                queue_push_tail(queue, c_ref!(var4).cast());
            });
            assert!(!queue_is_empty(queue).as_bool());
            assert!(queue_pop_head(queue) == c_ref!(var1).cast());
            assert!(queue_pop_head(queue) == c_ref!(var2).cast());
            assert!(queue_pop_head(queue) == c_ref!(var3).cast());
            assert!(queue_pop_head(queue) == c_ref!(var4).cast());
            assert!(queue_pop_tail(queue) == c_ref!(var4).cast());
            assert!(queue_pop_tail(queue) == c_ref!(var3).cast());
            assert!(queue_pop_tail(queue) == c_ref!(var2).cast());
            assert!(queue_pop_tail(queue) == c_ref!(var1).cast());
            queue_free(queue);
            test_no_memory_leak!();
        }}
    
        #[test]
        fn test_queue_pop_tail() {
        unsafe {
            let mut queue: Ptr<Queue>;
            queue = queue_new();
            assert!(queue_pop_tail(queue) == null!());
            queue_free(queue);
            queue = generate_queue();
            while !queue_is_empty(queue).as_bool() {
                assert!(queue_pop_tail(queue) == c_ref!(var1).cast());
                assert!(queue_pop_tail(queue) == c_ref!(var2).cast());
                assert!(queue_pop_tail(queue) == c_ref!(var3).cast());
                assert!(queue_pop_tail(queue) == c_ref!(var4).cast());
            }
            assert!(queue_pop_tail(queue) == null!());
            queue_free(queue);
            test_no_memory_leak!();
        }}
    
        fn test_queue_peek_tail() {
        unsafe {
            let mut queue: Ptr<Queue>;
            queue = queue_new();
            assert!(queue_peek_tail(queue) == null!());
            queue_free(queue);
            queue = generate_queue();
            while !queue_is_empty(queue).as_bool() {
                assert!(queue_peek_tail(queue) == c_ref!(var1).cast());
                assert!(queue_pop_tail(queue) == c_ref!(var1).cast());
                assert!(queue_peek_tail(queue) == c_ref!(var2).cast());
                assert!(queue_pop_tail(queue) == c_ref!(var2).cast());
                assert!(queue_peek_tail(queue) == c_ref!(var3).cast());
                assert!(queue_pop_tail(queue) == c_ref!(var3).cast());
                assert!(queue_peek_tail(queue) == c_ref!(var4).cast());
                assert!(queue_pop_tail(queue) == c_ref!(var4).cast());
            }
            assert!(queue_peek_tail(queue) == null!());
            queue_free(queue);
            test_no_memory_leak!();
        }}
    
        #[test]
        fn test_queue_is_empty() {
        unsafe {
            let mut queue: Ptr<Queue>;
            queue = queue_new();
            assert!(queue_is_empty(queue).as_bool());
            queue_push_head(queue, c_ref!(var1).cast());
            assert!(!queue_is_empty(queue).as_bool());
            queue_pop_head(queue);
            assert!(queue_is_empty(queue).as_bool());
            queue_push_tail(queue, c_ref!(var1).cast());
            assert!(!queue_is_empty(queue).as_bool());
            queue_pop_tail(queue);
            assert!(queue_is_empty(queue).as_bool());
            queue_free(queue);
            test_no_memory_leak!();
        }}

    fn generate_list() -> Ptr<ListEntry> {
        unsafe {
            let mut list: Ptr<ListEntry> = null!();
            assert!(list_append(c_ref!(list), c_ref!(vari1).cast()) != null!());
            assert!(list_append(c_ref!(list), c_ref!(vari2).cast()) != null!());
            assert!(list_append(c_ref!(list), c_ref!(vari3).cast()) != null!());
            assert!(list_append(c_ref!(list), c_ref!(vari4).cast()) != null!());
            return list;
        }
    }

    fn check_list_integrity(mut list: Ptr<ListEntry>) {
        let mut prev: Ptr<ListEntry> = null!();
        let mut rover: Ptr<ListEntry> = list;
        while rover != null!() {
            assert!(list_prev(rover) == prev);
            prev = rover;
            rover = list_next(rover);
        }
    }

    #[test]
    fn test_list_append() {
    unsafe {
        let mut list: Ptr<ListEntry> = null!();
        assert!(list_append(c_ref!(list), c_ref!(vari1).cast()) != null!());
        check_list_integrity(list);
        assert!(list_append(c_ref!(list), c_ref!(vari2).cast()) != null!());
        check_list_integrity(list);
        assert!(list_append(c_ref!(list), c_ref!(vari3).cast()) != null!());
        check_list_integrity(list);
        assert!(list_append(c_ref!(list), c_ref!(vari4).cast()) != null!());
        check_list_integrity(list);
        assert!(list_length(list) == 4);
        assert!(list_nth_data(list, 0) == c_ref!(vari1).cast());
        assert!(list_nth_data(list, 1) == c_ref!(vari2).cast());
        assert!(list_nth_data(list, 2) == c_ref!(vari3).cast());
        assert!(list_nth_data(list, 3) == c_ref!(vari4).cast());
        list_free(list);
        test_no_memory_leak!();
    }}

    #[test]
    fn test_list_prepend() {
    unsafe {
        let mut list: Ptr<ListEntry> = null!();
        assert!(list_prepend(c_ref!(list), c_ref!(vari1).cast()) != null!());
        check_list_integrity(list);
        assert!(list_prepend(c_ref!(list), c_ref!(vari2).cast()) != null!());
        check_list_integrity(list);
        assert!(list_prepend(c_ref!(list), c_ref!(vari3).cast()) != null!());
        check_list_integrity(list);
        assert!(list_prepend(c_ref!(list), c_ref!(vari4).cast()) != null!());
        check_list_integrity(list);
        assert!(list_nth_data(list, 0) == c_ref!(vari4).cast());
        assert!(list_nth_data(list, 1) == c_ref!(vari3).cast());
        assert!(list_nth_data(list, 2) == c_ref!(vari2).cast());
        assert!(list_nth_data(list, 3) == c_ref!(vari1).cast());
        list_free(list);
        test_no_memory_leak!();
    }}

    #[test]
    fn test_list_free() {
        let mut list: Ptr<ListEntry>;
        list = generate_list();
        list_free(list);
        list_free(null!());
        test_no_memory_leak!();
    }

    #[test]
    fn test_list_next() {
    unsafe {
        let mut list: Ptr<ListEntry>;
        let mut rover: Ptr<ListEntry>;
        list = generate_list();
        rover = list;
        assert!(list_data(rover) == c_ref!(vari1).cast());
        rover = list_next(rover);
        assert!(list_data(rover) == c_ref!(vari2).cast());
        rover = list_next(rover);
        assert!(list_data(rover) == c_ref!(vari3).cast());
        rover = list_next(rover);
        assert!(list_data(rover) == c_ref!(vari4).cast());
        rover = list_next(rover);
        assert!(rover == null!());
        list_free(list);
        test_no_memory_leak!();
    }}

    #[test]
    fn test_list_nth_entry() {
    unsafe {
        let mut list: Ptr<ListEntry>;
        let mut entry: Ptr<ListEntry>;
        list = generate_list();
        entry = list_nth_entry(list, 0);
        assert!(list_data(entry) == c_ref!(vari1).cast());
        entry = list_nth_entry(list, 1);
        assert!(list_data(entry) == c_ref!(vari2).cast());
        entry = list_nth_entry(list, 2);
        assert!(list_data(entry) == c_ref!(vari3).cast());
        entry = list_nth_entry(list, 3);
        assert!(list_data(entry) == c_ref!(vari4).cast());
        entry = list_nth_entry(list, 4);
        assert!(entry == null!());
        entry = list_nth_entry(list, 400);
        assert!(entry == null!());
        list_free(list);
        test_no_memory_leak!();
    }}

    #[test]
    fn test_list_nth_data() {
    unsafe {
        let mut list: Ptr<ListEntry>;
        list = generate_list();
        assert!(list_nth_data(list, 0) == c_ref!(vari1).cast());
        assert!(list_nth_data(list, 1) == c_ref!(vari2).cast());
        assert!(list_nth_data(list, 2) == c_ref!(vari3).cast());
        assert!(list_nth_data(list, 3) == c_ref!(vari4).cast());
        assert!(list_nth_data(list, 4) == null!());
        assert!(list_nth_data(list, 400) == null!());
        list_free(list);
        test_no_memory_leak!();
    }}

    #[test]
    fn test_list_length() {
    unsafe {
        let mut list: Ptr<ListEntry>;
        list = generate_list();
        assert!(list_length(list) == 4);
        assert!(list_prepend(c_ref!(list), c_ref!(vari1).cast()) != null!());
        assert!(list_length(list) == 5);
        list_free(list);
        assert!(list_length(null!()) == 0);
        test_no_memory_leak!();
    }}

    #[test]
    fn test_list_remove_entry() {
    unsafe {
        let mut empty_list: Ptr<ListEntry> = null!();
        let mut list: Ptr<ListEntry>;
        let mut entry: Ptr<ListEntry>;
        list = generate_list();
        entry = list_nth_entry(list, 2);
        assert!(list_remove_entry(c_ref!(list), entry) != 0);
        assert!(list_length(list) == 3);
        check_list_integrity(list);
        entry = list_nth_entry(list, 0);
        assert!(list_remove_entry(c_ref!(list), entry) != 0);
        assert!(list_length(list) == 2);
        check_list_integrity(list);
        assert!(list_remove_entry(c_ref!(list), null!()) == 0);
        assert!(list_remove_entry(c_ref!(empty_list), null!()) == 0);
        list_free(list);
        list = null!();
        assert!(list_append(c_ref!(list), c_ref!(vari1).cast()) != null!());
        assert!(list != null!());
        assert!(list_remove_entry(c_ref!(list), list) != 0);
        assert!(list == null!());
        list = generate_list();
        entry = list_nth_entry(list, 3);
        assert!(list_remove_entry(c_ref!(list), entry) != 0);
        check_list_integrity(list);
        list_free(list);
        test_no_memory_leak!();
    }}

    #[test]
    fn test_list_remove_data() {
    unsafe {
        let mut entries: Array<i32, 13> = arr![89, 4, 23, 42, 4, 16, 15, 4, 8, 99, 50, 30, 4];
        let num_entries: u32 = entries.len() as u32;
        let mut val: i32;
        let mut list: Ptr<ListEntry> = null!();
        let mut i: u32;
        c_for!(i = 0; i < num_entries; i += 1; {
            assert!(list_prepend(c_ref!(list), c_ref!(entries[i]).cast()) != null!());
        });
        val = 0;
        assert!(list_remove_data(c_ref!(list), func!(int_equal), c_ref!(val).cast()) == 0);
        val = 56;
        assert!(list_remove_data(c_ref!(list), func!(int_equal), c_ref!(val).cast()) == 0);
        check_list_integrity(list);
        val = 8;
        assert!(list_remove_data(c_ref!(list), func!(int_equal), c_ref!(val).cast()) == 1);
        assert!(list_length(list) == num_entries as u32 - 1);
        check_list_integrity(list);
        val = 4;
        assert!(list_remove_data(c_ref!(list), func!(int_equal), c_ref!(val).cast()) == 4);
        assert!(list_length(list) == num_entries as u32 - 5);
        check_list_integrity(list);
        val = 89;
        assert!(list_remove_data(c_ref!(list), func!(int_equal), c_ref!(val).cast()) == 1);
        assert!(list_length(list) == num_entries as u32 - 6);
        check_list_integrity(list);
        list_free(list);
        test_no_memory_leak!();
    }}

    #[test]
    fn test_list_sort() {
    unsafe {
        let mut list: Ptr<ListEntry> = null!();
        let mut entries: Array<i32, 13> = arr![89, 4, 23, 42, 4, 16, 15, 4, 8, 99, 50, 30, 4];
        let sorted: Array<i32, 13> = arr![4, 4, 4, 4, 8, 15, 16, 23, 30, 42, 50, 89, 99];
        let num_entries: u32 = entries.len() as u32;
        let mut i: u32;
        c_for!(i = 0; i < num_entries; i += 1; {
            assert!(list_prepend(c_ref!(list), c_ref!(entries[i]).cast()) != null!());
        });
        list_sort(c_ref!(list), func!(int_compare));
        assert!(list_length(list) == num_entries as u32);
        c_for!(i = 0; i < num_entries; i += 1; {
            let mut value: Ptr<i32>;
            value = list_nth_data(list, i).cast();
            assert!(*value == sorted[i]);
        });
        list_free(list);
        list = null!();
        list_sort(c_ref!(list), func!(int_compare));
        assert!(list == null!());
        test_no_memory_leak!();
    }}

    #[test]
    fn test_list_find_data() {
    unsafe {
        let mut entries: Array<i32, 10> = arr![89, 23, 42, 16, 15, 4, 8, 99, 50, 30];
        let num_entries: u32 = entries.len() as u32;
        let mut list: Ptr<ListEntry>;
        let mut result: Ptr<ListEntry>;
        let mut i: u32;
        let mut val: i32;
        let mut data: Ptr<i32>;
        list = null!();
        c_for!(i = 0; i < num_entries; i += 1; {
            assert!(list_append(c_ref!(list), c_ref!(entries[i]).cast()) != null!());
        });
        c_for!(i = 0; i < num_entries; i += 1; {
            val = entries[i];
            result = list_find_data(list, func!(int_equal), c_ref!(val).cast());
            assert!(result != null!());
            data = list_data(result).cast();
            assert!(*data == val);
        });
        val = 0;
        assert!(list_find_data(list, func!(int_equal), c_ref!(val).cast()) == null!());
        val = 56;
        assert!(list_find_data(list, func!(int_equal), c_ref!(val).cast()) == null!());
        list_free(list);
        test_no_memory_leak!();
    }}

    #[test]
    fn test_list_to_array() {
    unsafe {
        let mut list: Ptr<ListEntry>;
        let mut array: Ptr<ListValue>;
        list = generate_list();
        array = list_to_array(list);
        assert!(array[0] == c_ref!(vari1).cast());
        assert!(array[1] == c_ref!(vari2).cast());
        assert!(array[2] == c_ref!(vari3).cast());
        assert!(array[3] == c_ref!(vari4).cast());
        c_free!(array);
        list_free(list);
        test_no_memory_leak!();
    }}

    #[test]
    fn test_list_iterate() {
    unsafe {
        let mut list: Ptr<ListEntry>;
        let mut iter: ListIterator = Default::default();
        let mut i: i32;
        let mut a: i32 = 0;
        let mut counter: i32;
        let mut data: Ptr<i32>;
        list = null!();
        c_for!(i = 0; i < 50; i += 1; {
            assert!(list_prepend(c_ref!(list), c_ref!(a).cast()) != null!());
        });
        counter = 0;
        list_iterate(c_ref!(list), c_ref!(iter));
        list_iter_remove(c_ref!(iter));
        while list_iter_has_more(c_ref!(iter)) != 0 {
            data = list_iter_next(c_ref!(iter)).cast();
            counter += 1;
            if counter % 2 == 0 {
                list_iter_remove(c_ref!(iter));
                list_iter_remove(c_ref!(iter));
            }
        }
        assert!(list_iter_next(c_ref!(iter)) == null!());
        list_iter_remove(c_ref!(iter));
        assert!(counter == 50);
        assert!(list_length(list) == 25);
        list_free(list);
        list = null!();
        counter = 0;
        list_iterate(c_ref!(list), c_ref!(iter));
        while list_iter_has_more(c_ref!(iter)) != 0 {
            data = list_iter_next(c_ref!(iter)).cast();
            counter += 1;
        }
        assert!(counter == 0);
        test_no_memory_leak!();
    }}

    #[test]
    fn test_list_iterate_bad_remove() {
    unsafe {
        let mut list: Ptr<ListEntry>;
        let mut iter: ListIterator = Default::default();
        let mut values: Array<i32, 49> = arr![0; 49];
        let mut i: i32;
        let mut val: Ptr<i32>;
        list = null!();
        c_for!(i = 0; i < 49; i += 1; {
            values[i] = i;
            assert!(list_prepend(c_ref!(list), c_ref!(values[i]).cast()) != null!());
        });
        list_iterate(c_ref!(list), c_ref!(iter));
        while list_iter_has_more(c_ref!(iter)) != 0 {
            val = list_iter_next(c_ref!(iter)).cast();
            if *val % 2 == 0 {
                assert!(list_remove_data(c_ref!(list), func!(int_equal), val.cast()) != 0);
                list_iter_remove(c_ref!(iter));
            }
        }
        list_free(list);
        test_no_memory_leak!();
    }}

    fn generate_hash_table() -> Ptr<HashTable> {
		let mut hash_table: Ptr<HashTable>;
        let mut buf: Array<u8, 32> = arr![0; 32];
		let mut value: CStr;
		let mut i: i32;
		hash_table = hash_table_new(func!(string_hash), func!(string_equal));
		c_for!(i = 0; i < 10000; i += 1; {
			c_sprintf!(buf, cstr!("{}"), i);
			value = c_strdup!(buf);
			hash_table_insert(hash_table, value, value);
		});
		hash_table_register_free_functions(hash_table, null!(), c_free!());
		return hash_table;
	}

    #[test]
	fn test_hash_table_new_free() {
		let mut hash_table: Ptr<HashTable>;
		hash_table = hash_table_new(func!(int_hash), func!(int_equal));
		assert!(hash_table != null!());
		hash_table_insert(hash_table, c_ref!(value1).cast(), c_ref!(value1).cast());
		hash_table_insert(hash_table, c_ref!(value2).cast(), c_ref!(value2).cast());
		hash_table_insert(hash_table, c_ref!(value3).cast(), c_ref!(value3).cast());
		hash_table_insert(hash_table, c_ref!(value4).cast(), c_ref!(value4).cast());
		hash_table_free(hash_table);
		test_no_memory_leak!();
	}

    #[test]
	fn test_hash_table_insert_lookup() {
		let mut hash_table: Ptr<HashTable>;
		let mut buf: Array<u8, 32> = arr![0; 32];
		let mut value: CStr;
		let mut i: i32;
		hash_table = generate_hash_table();
		assert!(hash_table_num_entries(hash_table) == 10000);
		c_for!(i = 0; i < 10000; i += 1; {
			c_sprintf!(buf, cstr!("{}"), i);
			value = hash_table_lookup(hash_table, buf.cast());
			assert!(c_strcmp!(value, buf) == 0);
		});
		c_sprintf!(buf, cstr!("{}"), -1);
		assert!(hash_table_lookup(hash_table, buf.cast()) == null!());
		c_sprintf!(buf, cstr!("{}"), 10000);
		assert!(hash_table_lookup(hash_table, buf.cast()) == null!());
		c_sprintf!(buf, cstr!("{}"), 12345);
		hash_table_insert(hash_table, buf.cast(), c_strdup!(cstr!("hello world")));
		value = hash_table_lookup(hash_table, buf.cast());
		assert!(c_strcmp!(value, cstr!("hello world")) == 0);
		hash_table_free(hash_table);
		test_no_memory_leak!();
	}

    #[test]
	fn test_hash_table_remove() {
		let mut hash_table: Ptr<HashTable>;
        let mut buf: Array<u8, 32> = arr![0; 32];
		hash_table = generate_hash_table();
		assert!(hash_table_num_entries(hash_table) == 10000);
		c_sprintf!(buf, cstr!("{}"), 5000);
		assert!(hash_table_lookup(hash_table, buf.cast()) != null!());
		hash_table_remove(hash_table, buf.cast());
		assert!(hash_table_num_entries(hash_table) == 10000 - 1);
		assert!(hash_table_lookup(hash_table, buf.cast()) == null!());
		c_sprintf!(buf, cstr!("{}"), -1);
		hash_table_remove(hash_table, buf.cast());
		assert!(hash_table_num_entries(hash_table) == 10000 - 1);
		hash_table_free(hash_table);
		test_no_memory_leak!();
	}

    #[test]
	fn test_hash_table_iterating() {
		let mut hash_table: Ptr<HashTable>;
		let mut iterator: HashTableIterator = Default::default();
		let mut count: i32;
		hash_table = generate_hash_table();
		count = 0;
		hash_table_iterate(hash_table, c_ref!(iterator));
		while hash_table_iter_has_more(c_ref!(iterator)).as_bool() {
			hash_table_iter_next(c_ref!(iterator));
			count += 1;
		}
		assert!(count == 10000);
		let pair = hash_table_iter_next(c_ref!(iterator));
		assert!(pair.value == HASH_TABLE_NULL!());
		hash_table_free(hash_table);

		let mut hash_table: Ptr<HashTable>;
		let mut iterator: HashTableIterator = Default::default();
		hash_table = hash_table_new(func!(int_hash), func!(int_equal));
		hash_table_iterate(hash_table, c_ref!(iterator));
		assert!(hash_table_iter_has_more(c_ref!(iterator)) == 0);
		hash_table_free(hash_table);
		test_no_memory_leak!();
	}

    #[test]
	fn test_hash_table_iterating_remove() {
		let mut hash_table: Ptr<HashTable>;
		let mut iterator: HashTableIterator = Default::default();
		let mut buf: Array<u8, 32> = arr![0; 32];
		let mut val: CStr;
		let mut pair: HashTablePair;
		let mut count: i32;
		let mut removed: u32;
		let mut i: i32;
		hash_table = generate_hash_table();
		count = 0;
		removed = 0;
		hash_table_iterate(hash_table, c_ref!(iterator));
		while hash_table_iter_has_more(c_ref!(iterator)).as_bool() {
			pair = hash_table_iter_next(c_ref!(iterator));
			val = pair.value;
			if (c_atoi!(val) % 100) == 0 {
				hash_table_remove(hash_table, val);
				removed += 1;
			}
			count += 1;
		}
		assert!(removed == 100);
		assert!(count == 10000);
		assert!(hash_table_num_entries(hash_table)
			== 10000 - removed);
		c_for!(i = 0; i < 10000; i += 1; {
			c_sprintf!(buf, cstr!("{}"), i);
			if i % 100 == 0 {
				assert!(hash_table_lookup(hash_table, buf.cast()) == null!());
			} else {
				assert!(hash_table_lookup(hash_table, buf.cast()) != null!());
			}
		});
		hash_table_free(hash_table);
		test_no_memory_leak!();
	}

    fn new_key(mut value: i32) -> Ptr<i32> {
        unsafe {
            let mut result: Ptr<i32>;
            result = c_malloc!(c_sizeof!(i32));
            *result = value;
            allocated_keys += 1;		
            result
        }
    }

    fn free_key(mut key: VoidPtr) {
        unsafe {
            c_free!(key);
            allocated_keys -= 1;
        }
    }

    fn new_value2(mut value: i32) -> Ptr<i32> {
        unsafe {
            let mut result: Ptr<i32>;
            result = c_malloc!(c_sizeof!(i32));
            *result = value;
            allocated_values += 1; 		
            return result
        }
    }

    fn free_value2(mut value: VoidPtr) {
        unsafe {
            c_free!(value);
            allocated_values -= 1;
        }
    }

    #[test]
	pub fn test_hash_table_free_functions() {
        unsafe {
            let mut hash_table: Ptr<HashTable>;
            let mut key: Ptr<i32>;
            let mut value: Ptr<i32>;
            let mut i: i32;
            hash_table = hash_table_new(func!(int_hash), func!(int_equal));
            hash_table_register_free_functions(hash_table, func!(free_key), func!(free_value2));
            allocated_values = 0;
            c_for!(i = 0; i < 10000; i += 1; {
                key = new_key(i);
                value = new_value2(99);
                hash_table_insert(hash_table, key.cast(), value.cast());
            });
            assert!(allocated_keys == 10000);
            assert!(allocated_values == 10000);
            i = 10000 / 2;
            hash_table_remove(hash_table, c_ref!(i).cast());
            assert!(allocated_keys == 10000 - 1);
            assert!(allocated_values == 10000 - 1);
            key = new_key(10000 / 3);
            value = new_value2(999);
            assert!(allocated_keys == 10000);
            assert!(allocated_values == 10000);
            hash_table_insert(hash_table, key.cast(), value.cast());
            assert!(allocated_keys == 10000 - 1);
            assert!(allocated_values == 10000 - 1);
            hash_table_free(hash_table);
            assert!(allocated_keys == 0);
            assert!(allocated_values == 0);
            test_no_memory_leak!();
        }
    }

    #[test]
	fn test_bloom_filter_new_free() {
		let mut filter: Ptr<BloomFilter>;
		filter = bloom_filter_new(128, func!(string_hash), 1);
		assert!(filter != null!());
		bloom_filter_free(filter);
		filter = bloom_filter_new(128, func!(string_hash), 64);
		assert!(filter != null!());
		bloom_filter_free(filter);
		filter = bloom_filter_new(128, func!(string_hash), 50000);
		assert!(filter == null!());
	}

	#[test]
	fn test_bloom_filter_insert_query() {
		let mut filter: Ptr<BloomFilter>;
		filter = bloom_filter_new(128, func!(string_hash), 4);
		assert!(bloom_filter_query(filter, cstr!("test 1")) == 0);
		assert!(bloom_filter_query(filter, cstr!("test 2")) == 0);
		bloom_filter_insert(filter, cstr!("test 1"));
		bloom_filter_insert(filter, cstr!("test 2"));
		assert!(bloom_filter_query(filter, cstr!("test 1")) != 0);
		assert!(bloom_filter_query(filter, cstr!("test 2")) != 0);
		bloom_filter_free(filter);
	}

	#[test]
	fn test_bloom_filter_read_load() {
		let mut filter1: Ptr<BloomFilter>;
		let mut filter2: Ptr<BloomFilter>;
		let mut state: Array<u8, 16> = arr![0; 16];
		filter1 = bloom_filter_new(128, func!(string_hash), 4);
		bloom_filter_insert(filter1, cstr!("test 1"));
		bloom_filter_insert(filter1, cstr!("test 2"));
		bloom_filter_read(filter1, state.cast());
		bloom_filter_free(filter1);
		filter2 = bloom_filter_new(128, func!(string_hash), 4);
		bloom_filter_load(filter2, state.cast());
		assert!(bloom_filter_query(filter2, cstr!("test 1")) != 0);
		assert!(bloom_filter_query(filter2, cstr!("test 2")) != 0);
		bloom_filter_free(filter2);
	}

	#[test]
	fn test_bloom_filter_intersection() {
		let mut filter1: Ptr<BloomFilter>;
		let mut filter2: Ptr<BloomFilter>;
		let mut result: Ptr<BloomFilter>;
		filter1 = bloom_filter_new(128, func!(string_hash), 4);
		bloom_filter_insert(filter1, cstr!("test 1"));
		bloom_filter_insert(filter1, cstr!("test 2"));
		filter2 = bloom_filter_new(128, func!(string_hash), 4);
		bloom_filter_insert(filter2, cstr!("test 1"));
		assert!(bloom_filter_query(filter2, cstr!("test 2")) == 0);
		result = bloom_filter_intersection(filter1, filter2);
		assert!(bloom_filter_query(result, cstr!("test 1")) != 0);
		assert!(bloom_filter_query(result, cstr!("test 2")) == 0);
        bloom_filter_free(filter1);
        bloom_filter_free(filter2);
		bloom_filter_free(result);
	}

	#[test]
	fn test_bloom_filter_union() {
		let mut filter1: Ptr<BloomFilter>;
		let mut filter2: Ptr<BloomFilter>;
		let mut result: Ptr<BloomFilter>;
		filter1 = bloom_filter_new(128, func!(string_hash), 4);
		bloom_filter_insert(filter1, cstr!("test 1"));
		filter2 = bloom_filter_new(128, func!(string_hash), 4);
		bloom_filter_insert(filter2, cstr!("test 2"));
		result = bloom_filter_union(filter1, filter2);
		assert!(bloom_filter_query(result, cstr!("test 1")) != 0);
		assert!(bloom_filter_query(result, cstr!("test 2")) != 0);
        bloom_filter_free(filter1);
        bloom_filter_free(filter2);
		bloom_filter_free(result);
	}

    #[test]
    fn test_binomial_heap_new_free() {
        let mut heap: Ptr<BinomialHeap>;
        let mut i: i32;

        c_for!(i = 0; i < 1000; i += 1; {
            heap = binomial_heap_new(BINOMIAL_HEAP_TYPE_MIN!(), func!(int_compare));
            binomial_heap_free(heap);
        });
        test_no_memory_leak!();
    }

	#[test]
	fn test_binomial_heap_insert() {
	unsafe {
		let mut heap: Ptr<BinomialHeap>;
		let mut i: i32;

		heap = binomial_heap_new(BINOMIAL_HEAP_TYPE_MIN!(), func!(int_compare));
		c_for!(i = 0; i < 1000; i += 1; {
			test_array3[i] = i;
			assert!(binomial_heap_insert(heap, c_ref!(test_array3[i]).cast()) != 0);
		});
		assert!(binomial_heap_num_entries(heap) == 1000 as u32);
		binomial_heap_free(heap);
		test_no_memory_leak!();
	}}

	#[test]
	fn test_min_heap() {
	unsafe {
		let mut heap: Ptr<BinomialHeap>;
		let mut val: Ptr<i32>;
		let mut i: i32;

		heap = binomial_heap_new(BINOMIAL_HEAP_TYPE_MIN!(), func!(int_compare));
		c_for!(i = 0; i < 1000; i += 1; {
			test_array3[i] = i;
			assert!(binomial_heap_insert(heap, c_ref!(test_array3[i]).cast()) != 0);
		});
		i = -1;
		while binomial_heap_num_entries(heap) > 0 {
			val = binomial_heap_pop(heap).cast();
			assert!(*val == i + 1);
			i = *val;
		}
		val = binomial_heap_pop(heap).cast();
		assert!(val == null!());
		binomial_heap_free(heap);
		test_no_memory_leak!();
	}}

	#[test]
	fn test_max_heap() {
	unsafe {
		let mut heap: Ptr<BinomialHeap>;
		let mut val: Ptr<i32>;
		let mut i: i32;
		heap = binomial_heap_new(BINOMIAL_HEAP_TYPE_MAX!(), func!(int_compare));
		c_for!(i = 0; i < 1000; i += 1; {
			test_array3[i] = i;
			assert!(binomial_heap_insert(heap, c_ref!(test_array3[i]).cast()) != 0);
		});
		i = 1000;
		while binomial_heap_num_entries(heap) > 0 {
			val = binomial_heap_pop(heap).cast();
			assert!(*val == i - 1);
			i = *val;
		}
		val = binomial_heap_pop(heap).cast();
		assert!(val == null!());
		binomial_heap_free(heap);
		test_no_memory_leak!();
	}}

    #[test]
    fn test_binary_heap_new_free() {
        let mut heap: Ptr<BinaryHeap>;
        let mut i: i32;
        c_for!(i = 0; i < 10000; i += 1; {
            heap = binary_heap_new(BINARY_HEAP_TYPE_MIN!(), func!(int_compare));
            binary_heap_free(heap);
            i += 1;
        });
    }

    #[test]
    fn test_binary_heap_insert() {
    unsafe {
        let mut heap: Ptr<BinaryHeap>;
        let mut i: i32;
        heap = binary_heap_new(BINARY_HEAP_TYPE_MIN!(), func!(int_compare));
        c_for!(i = 0; i < 10000; i += 1; {
            test_array2[i] = i;
            assert!(binary_heap_insert(heap, c_ref!(test_array2[i]).cast()) != 0);
        });
        assert!(binary_heap_num_entries(heap) == 10000);
        binary_heap_free(heap);
    }}

    #[test]
    fn test_min_heap2() {
    unsafe {
        let mut heap: Ptr<BinaryHeap>;
        let mut val: Ptr<i32>;
        let mut i: i32;
        heap = binary_heap_new(BINARY_HEAP_TYPE_MIN!(), func!(int_compare));
        c_for!(i = 0; i < 10000; i += 1; {
            test_array2[i] = i;
            assert!(binary_heap_insert(heap, c_ref!(test_array2[i]).cast()) != 0);
        });
        i = -1;
        while binary_heap_num_entries(heap) > 0 {
            val = binary_heap_pop(heap).cast();
            assert!(*val == i + 1);
            i = *val;
        }
        assert!(binary_heap_num_entries(heap) == 0);
        assert!(binary_heap_pop(heap) == BINARY_HEAP_NULL!());
        binary_heap_free(heap);
    }}

    #[test]
    fn test_max_heap2() {
    unsafe {
        let mut heap: Ptr<BinaryHeap>;
        let mut val: Ptr<i32>;
        let mut i: i32;
        heap = binary_heap_new(BINARY_HEAP_TYPE_MAX!(), func!(int_compare));
        c_for!(i = 0; i < 10000; i += 1; {
            test_array2[i] = i;
            assert!(binary_heap_insert(heap, c_ref!(test_array2[i]).cast()) != 0);
        });
        i = 10000;
        while binary_heap_num_entries(heap) > 0 {
            val = binary_heap_pop(heap).cast();
            assert!(*val == i - 1);
            i = *val;
        }
        binary_heap_free(heap);
    }}

    fn find_subtree_height2(mut node: Ptr<AVLTreeNode>) -> i32 {
        let mut left_subtree: Ptr<AVLTreeNode>;
        let mut right_subtree: Ptr<AVLTreeNode>;
        let mut left_height: i32;
        let mut right_height: i32;

        if node == null!() {
            return 0;
        }

        left_subtree = avl_tree_node_child(node, AVL_TREE_NODE_LEFT!());
        right_subtree = avl_tree_node_child(node, AVL_TREE_NODE_RIGHT!());
        left_height = find_subtree_height2(left_subtree);
        right_height = find_subtree_height2(right_subtree);

        if left_height > right_height {
            return left_height + 1;
        } else {
            return right_height + 1;
        }
    }

    fn validate_subtree(mut node: Ptr<AVLTreeNode>) -> i32 {
        unsafe {
            let mut left_node: Ptr<AVLTreeNode>;
            let mut right_node: Ptr<AVLTreeNode>;
            let mut left_height: i32;
            let mut right_height: i32;
            let mut key: Ptr<i32>;
            if node == null!() {
                return 0;
            }
            left_node = avl_tree_node_child(node, AVL_TREE_NODE_LEFT!());
            right_node = avl_tree_node_child(node, AVL_TREE_NODE_RIGHT!());
            if left_node != null!() {
                assert!(avl_tree_node_parent(left_node) == node);
            }
            if right_node != null!() {
                assert!(avl_tree_node_parent(right_node) == node);
            }
            left_height = validate_subtree(left_node);
            key = avl_tree_node_key(node).cast();
            assert!(*key > COUNTER);
            COUNTER = *key;
            right_height = validate_subtree(right_node);
            assert!(avl_tree_subtree_height(left_node) == left_height);
            assert!(avl_tree_subtree_height(right_node) == right_height);
            assert!(left_height - right_height < 2 && right_height - left_height < 2);
            if left_height > right_height {
                return left_height + 1;
            } else {
                return right_height + 1;
            }
        }
    }
    
    fn validate_tree2(mut tree: Ptr<AVLTree>) {
        unsafe {
            let mut root_node: Ptr<AVLTreeNode>;
            let mut height: i32;
    
            root_node = avl_tree_root_node(tree);
    
            if root_node != null!() {
                height = find_subtree_height2(root_node);
                assert!(avl_tree_subtree_height(root_node) == height);
            }
            
                COUNTER = -1;
            
            validate_subtree(root_node);
        }
    }

    fn create_tree2() -> Ptr<AVLTree> {
        unsafe {
            let mut tree: Ptr<AVLTree>;
            let mut i: i32;
            tree = avl_tree_new(func!(int_compare));
            c_for!(i = 0; i < 1000; i += 1; {
                test_array[i] = i;
                avl_tree_insert(tree, c_ref!(test_array[i]).cast(), c_ref!(test_array[i]).cast());
            });
            return tree;
        }
    }

    #[test]
    fn test_avl_tree_new() {
        let mut tree: Ptr<AVLTree>;
        tree = avl_tree_new(func!(int_compare));
        assert!(tree != null!());
        assert!(avl_tree_root_node(tree) == null!());
        assert!(avl_tree_num_entries(tree) == 0);
        avl_tree_free(tree);
        test_no_memory_leak!();
    }

    #[test]
    fn test_avl_tree_insert_lookup() {
        unsafe {
            let mut tree: Ptr<AVLTree>;
            let mut node: Ptr<AVLTreeNode>;
            let mut i: i32;
            let mut value: Ptr<i32>;
            tree = avl_tree_new(func!(int_compare));
            c_for!(i = 0; i < 1000; i += 1; {
                test_array[i] = i;
                avl_tree_insert(tree, c_ref!(test_array[i]).cast(), c_ref!(test_array[i]).cast());
                assert!(avl_tree_num_entries(tree) == (i + 1) as u32);
                validate_tree2(tree);
            });
            assert!(avl_tree_root_node(tree) != null!());
            c_for!(i = 0; i < 1000; i += 1; {
                node = avl_tree_lookup_node(tree, c_ref!(i).cast());
                assert!(node != null!());
                value = avl_tree_node_key(node).cast();
                assert!(*value == i);
                value = avl_tree_node_value(node).cast();
                assert!(*value == i);
            });
            i = 1000 + 100;
            assert!(avl_tree_lookup_node(tree, c_ref!(i).cast()) == null!());
            avl_tree_free(tree);
            test_no_memory_leak!();
        }
    }

    #[test]
    fn test_avl_tree_child() {
        let mut tree: Ptr<AVLTree>;
        let mut root: Ptr<AVLTreeNode>;
        let mut left: Ptr<AVLTreeNode>;
        let mut right: Ptr<AVLTreeNode>;
        let mut values: Array<i32, 3> = arr![1, 2, 3];
        let mut p: Ptr<i32>;
        let mut i: i32;
        tree = avl_tree_new(func!(int_compare));
        c_for!(i = 0; i < 3; i += 1; {
            avl_tree_insert(tree, c_ref!(values[i]).cast(), c_ref!(values[i]).cast());
        });
        root = avl_tree_root_node(tree);
        p = avl_tree_node_value(root).cast();
        assert!(*p == 2);
        left = avl_tree_node_child(root, AVL_TREE_NODE_LEFT!());
        p = avl_tree_node_value(left).cast();
        assert!(*p == 1);
        right = avl_tree_node_child(root, AVL_TREE_NODE_RIGHT!());
        p = avl_tree_node_value(right).cast();
        assert!(*p == 3);
        assert!(avl_tree_node_child(root, 10000) == null!());
        assert!(avl_tree_node_child(root, 2) == null!());
        avl_tree_free(tree);
        test_no_memory_leak!();
    }

    #[test]
    fn test_avl_tree_free() {
        let mut tree: Ptr<AVLTree>;
        tree = avl_tree_new(func!(int_compare));
        avl_tree_free(tree);
        tree = create_tree2();
        avl_tree_free(tree);
        test_no_memory_leak!();
    }

    #[test]
    fn test_avl_tree_lookup() {
    unsafe {
        let mut tree: Ptr<AVLTree>;
        let mut i: i32;
        let mut value: Ptr<i32>;
        tree = create_tree2();
        c_for!(i = 0; i < 1000; i += 1; {
            value = avl_tree_lookup(tree, c_ref!(i).cast()).cast();
            assert!(value != null!());
            assert!(*value == i);
        });
        i = -1;
        assert!(avl_tree_lookup(tree, c_ref!(i).cast()) == null!());
        i = 1000 + 1;
        assert!(avl_tree_lookup(tree, c_ref!(i).cast()) == null!());
        i = 8724897;
        assert!(avl_tree_lookup(tree, c_ref!(i).cast()) == null!());
        avl_tree_free(tree);
        test_no_memory_leak!();
    }}

    #[test]
    fn test_avl_tree_remove() {
        unsafe {
            let mut tree: Ptr<AVLTree>;
            let mut i: i32;
            let mut x: i32;
            let mut y: i32;
            let mut z: i32;
            let mut value: i32;
            let mut expected_entries: u32;
            tree = create_tree2();
            i = 1000 + 100;
            assert!(avl_tree_remove(tree, c_ref!(i).cast()) == 0);
            i = -1;
            assert!(avl_tree_remove(tree, c_ref!(i).cast()) == 0);
            expected_entries = 1000 as u32;
            c_for!(x = 0; x < 10; x += 1; {
                c_for!(y = 0; y < 10; y += 1; {
                    c_for!(z = 0; z < 10; z += 1; {
                        value = z * 100 + (9 - y) * 10 + x;
                        assert!(avl_tree_remove(tree, c_ref!(value).cast()) != 0);
                        validate_tree2(tree);
                        expected_entries -= 1;
                        assert!(avl_tree_num_entries(tree) == expected_entries);
                    });
                });
            });
            assert!(avl_tree_root_node(tree) == null!());
            avl_tree_free(tree);
            test_no_memory_leak!();
        }
    }

    #[test]
    fn test_avl_tree_to_array() {
        let mut tree: Ptr<AVLTree>;
        let mut entries: Array<i32, 10> = arr![89, 23, 42, 4, 16, 15, 8, 99, 50, 30];
        let mut sorted: Array<i32, 10> = arr![4, 8, 15, 16, 23, 30, 42, 50, 89, 99];
        let mut num_entries: usize = entries.len();
        let mut i: usize;
        let mut array: Ptr<AVLTreeValue>;
        tree = avl_tree_new(func!(int_compare));
        c_for!(i = 0; i < num_entries; i += 1; {
            avl_tree_insert(tree, c_ref!(entries[i]).cast(), null!());
        });
        assert!(avl_tree_num_entries(tree) == num_entries as u32);
        array = avl_tree_to_array(tree);
        c_for!(i = 0; i < num_entries; i += 1; {
            assert!(*array[i] == sorted[i].cast());
        });
        c_free!(array);
        avl_tree_free(tree);
        test_no_memory_leak!();
    }

    fn generate_arraylist() -> Ptr<ArrayList> {
        unsafe {
            let mut arraylist: Ptr<ArrayList>;
            let mut i;
            arraylist = arraylist_new(0);
            c_for!(i = 0; i < 4; i += 1; {
                arraylist_append(arraylist, c_ref!(variable1).cast());
                arraylist_append(arraylist, c_ref!(variable2).cast());
                arraylist_append(arraylist, c_ref!(variable3).cast());
                arraylist_append(arraylist, c_ref!(variable4).cast());
            });
            arraylist
        }
    }

        #[test]
        fn test_arraylist_new_free() {
        unsafe {
            let mut arraylist: Ptr<ArrayList>;
            arraylist = arraylist_new(0);
            assert!(arraylist != null!());
            arraylist_free(arraylist);
            arraylist = arraylist_new(10);
            assert!(arraylist != null!());
            arraylist_free(arraylist);
            arraylist_free(null!());
        }
    }

    #[test]
    fn test_arraylist_append() {
        unsafe {
            let mut arraylist: Ptr<ArrayList>;
            let mut i;
            arraylist = arraylist_new(0);
            assert!(arraylist_append(arraylist, c_ref!(variable1).cast()) != 0);
            assert!(arraylist.length == 1);
            assert!(arraylist_append(arraylist, c_ref!(variable2).cast()) != 0);
            assert!(arraylist.length == 2);
            assert!(arraylist_append(arraylist, c_ref!(variable3).cast()) != 0);
            assert!(arraylist.length == 3);
            assert!(arraylist_append(arraylist, c_ref!(variable4).cast()) != 0);
            assert!(arraylist.length == 4);
            assert!(arraylist.data[0] == c_ref!(variable1).cast());
            assert!(arraylist.data[1] == c_ref!(variable2).cast());
            assert!(arraylist.data[2] == c_ref!(variable3).cast());
            assert!(arraylist.data[3] == c_ref!(variable4).cast());
            c_for!(i = 0; i < 10000; i += 1; {
                assert!(arraylist_append(arraylist, null!()) != 0);
            });
            arraylist_free(arraylist);
        }
    }

    #[test]
    fn test_arraylist_prepend() {
    unsafe {
        let mut arraylist: Ptr<ArrayList>;
        let mut i;
        arraylist = arraylist_new(0);
        assert!(arraylist.length == 0);
        assert!(arraylist_prepend(arraylist, c_ref!(variable1).cast()) != 0);
        assert!(arraylist.length == 1);
        assert!(arraylist_prepend(arraylist, c_ref!(variable2).cast()) != 0);
        assert!(arraylist.length == 2);
        assert!(arraylist_prepend(arraylist, c_ref!(variable3).cast()) != 0);
        assert!(arraylist.length == 3);
        assert!(arraylist_prepend(arraylist, c_ref!(variable4).cast()) != 0);
        assert!(arraylist.length == 4);
        assert!(arraylist.data[0] == c_ref!(variable4).cast());
        assert!(arraylist.data[1] == c_ref!(variable3).cast());
        assert!(arraylist.data[2] == c_ref!(variable2).cast());
        assert!(arraylist.data[3] == c_ref!(variable1).cast());
        c_for!(i = 0; i < 10000; i += 1; {
            assert!(arraylist_prepend(arraylist, null!()) != 0);
        });
        arraylist_free(arraylist);
    }}

    #[test]
    fn test_arraylist_insert() {
    unsafe {
        let mut arraylist: Ptr<ArrayList>;
        let mut i;
        arraylist = generate_arraylist();
        assert!(arraylist.length == 16);
        assert!(arraylist_insert(arraylist, 17, c_ref!(variable1).cast()) == 0);
        assert!(arraylist.length == 16);
        assert!(arraylist.length == 16);
        assert!(arraylist.data[4] == c_ref!(variable1).cast());
        assert!(arraylist.data[5] == c_ref!(variable2).cast());
        assert!(arraylist.data[6] == c_ref!(variable3).cast());
        assert!(arraylist_insert(arraylist, 5, c_ref!(variable4).cast()) != 0);
        assert!(arraylist.length == 17);
        assert!(arraylist.data[4] == c_ref!(variable1).cast());
        assert!(arraylist.data[5] == c_ref!(variable4).cast());
        assert!(arraylist.data[6] == c_ref!(variable2).cast());
        assert!(arraylist.data[7] == c_ref!(variable3).cast());
        assert!(arraylist.data[0] == c_ref!(variable1).cast());
        assert!(arraylist.data[1] == c_ref!(variable2).cast());
        assert!(arraylist.data[2] == c_ref!(variable3).cast());
        assert!(arraylist_insert(arraylist, 0, c_ref!(variable4).cast()) != 0);
        assert!(arraylist.length == 18);
        assert!(arraylist.data[0] == c_ref!(variable4).cast());
        assert!(arraylist.data[1] == c_ref!(variable1).cast());
        assert!(arraylist.data[2] == c_ref!(variable2).cast());
        assert!(arraylist.data[3] == c_ref!(variable3).cast());
        assert!(arraylist.data[15] == c_ref!(variable2).cast());
        assert!(arraylist.data[16] == c_ref!(variable3).cast());
        assert!(arraylist.data[17] == c_ref!(variable4).cast());
        assert!(arraylist_insert(arraylist, 18, c_ref!(variable1).cast()) != 0);
        assert!(arraylist.length == 19);
        assert!(arraylist.data[15] == c_ref!(variable2).cast());
        assert!(arraylist.data[16] == c_ref!(variable3).cast());
        assert!(arraylist.data[17] == c_ref!(variable4).cast());
        assert!(arraylist.data[18] == c_ref!(variable1).cast());
        c_for!(i = 0; i < 10000; i += 1; {
            arraylist_insert(arraylist, 10, c_ref!(variable1).cast());
        });
        arraylist_free(arraylist);
    }}

    #[test]
    fn test_arraylist_remove_range() {
    unsafe {
        let mut arraylist: Ptr<ArrayList>;
        arraylist = generate_arraylist();
        assert!(arraylist.length == 16);
        assert!(arraylist.data[3] == c_ref!(variable4).cast());
        assert!(arraylist.data[4] == c_ref!(variable1).cast());
        assert!(arraylist.data[5] == c_ref!(variable2).cast());
        assert!(arraylist.data[6] == c_ref!(variable3).cast());
        arraylist_remove_range(arraylist, 4, 3);
        assert!(arraylist.length == 13);
        assert!(arraylist.data[3] == c_ref!(variable4).cast());
        assert!(arraylist.data[4] == c_ref!(variable4).cast());
        assert!(arraylist.data[5] == c_ref!(variable1).cast());
        assert!(arraylist.data[6] == c_ref!(variable2).cast());
        arraylist_remove_range(arraylist, 10, 10);
        arraylist_remove_range(arraylist, 0, 16);
        assert!(arraylist.length == 13);
        arraylist_free(arraylist);
    }}

    #[test]
    fn test_arraylist_remove() {
    unsafe {
        let mut arraylist: Ptr<ArrayList>;
        arraylist = generate_arraylist();
        assert!(arraylist.length == 16);
        assert!(arraylist.data[3] == c_ref!(variable4).cast());
        assert!(arraylist.data[4] == c_ref!(variable1).cast());
        assert!(arraylist.data[5] == c_ref!(variable2).cast());
        assert!(arraylist.data[6] == c_ref!(variable3).cast());
        arraylist_remove(arraylist, 4);
        assert!(arraylist.length == 15);
        assert!(arraylist.data[3] == c_ref!(variable4).cast());
        assert!(arraylist.data[4] == c_ref!(variable2).cast());
        assert!(arraylist.data[5] == c_ref!(variable3).cast());
        assert!(arraylist.data[6] == c_ref!(variable4).cast());
        arraylist_remove(arraylist, 15);
        assert!(arraylist.length == 15);
        arraylist_free(arraylist);
    }}

    #[test]
    fn test_arraylist_index_of() {
    unsafe {
        let mut entries: Array<i32, 10> = arr![89, 4, 23, 42, 16, 15, 8, 99, 50, 30];
        let num_entries: i32;
        let mut arraylist: Ptr<ArrayList>;
        let mut i: i32;
        let mut index: i32;
        let mut val: i32;
        num_entries = entries.len() as i32;
        arraylist = arraylist_new(0);
        c_for!(i = 0; i < num_entries; i += 1; {
            arraylist_append(arraylist, c_ref!(entries[i]).cast());
        });
        c_for!(i = 0; i < num_entries; i += 1; {
            val = entries[i];
            index = arraylist_index_of(arraylist, func!(int_equal), c_ref!(val).cast());
            assert!(index == i);
        });
        val = 0;
        assert!(arraylist_index_of(arraylist, func!(int_equal), c_ref!(val).cast()) < 0);
        val = 57;
        assert!(arraylist_index_of(arraylist, func!(int_equal), c_ref!(val).cast()) < 0);
        arraylist_free(arraylist);
    }}

    #[test]
    fn test_arraylist_clear() {
    unsafe {
        let mut arraylist: Ptr<ArrayList>;
        arraylist = arraylist_new(0);
        arraylist_clear(arraylist);
        assert!(arraylist.length == 0);
        arraylist_append(arraylist, c_ref!(variable1).cast());
        arraylist_append(arraylist, c_ref!(variable2).cast());
        arraylist_append(arraylist, c_ref!(variable3).cast());
        arraylist_append(arraylist, c_ref!(variable4).cast());
        arraylist_clear(arraylist);
        assert!(arraylist.length == 0);
        arraylist_free(arraylist);
    }}

    #[test]
    fn test_arraylist_sort() {
    unsafe {
        let mut entries: Array<i32, 13> = arr![89, 4, 23, 42, 4, 16, 15, 4, 8, 99, 50, 30, 4];
        let mut sorted: Array<i32, 13> = arr![4, 4, 4, 4, 8, 15, 16, 23, 30, 42, 50, 89, 99];
        let num_entries: u32 = entries.len() as u32;
        let mut i: u32;
        let mut arraylist: Ptr<ArrayList>;
        arraylist = arraylist_new(10);
        c_for!(i = 0; i < num_entries; i += 1; {
            arraylist_prepend(arraylist, c_ref!(entries[i]).cast());
        });
        arraylist_sort(arraylist, func!(int_compare));
        assert!(arraylist.length == num_entries);
        c_for!(i = 0; i < num_entries; i += 1; {
            let mut value: i32;
            value = (*arraylist.data[i]).cast();
            assert!(value == sorted[i]);
        });
        arraylist_free(arraylist);
        arraylist = arraylist_new(5);
        arraylist_sort(arraylist, func!(int_compare));
        assert!(arraylist.length == 0);
        arraylist_free(arraylist);
        arraylist = arraylist_new(5);
        arraylist_prepend(arraylist, c_ref!(entries[0]).cast());
        arraylist_sort(arraylist, func!(int_compare));
        assert!(arraylist.length == 1);
        assert!(arraylist.data[0] == c_ref!(entries[0]).cast());
        arraylist_free(arraylist);
    }}

    #[test]
    fn test_string_hash() {
        let mut test1 = cstr!("this is a test");
        let mut test2 = cstr!("this is a tesu");
        let mut test3 = cstr!("this is a test ");
        let mut test4 = cstr!("this is a test");
        let mut test5 = cstr!("This is a test");

        assert!(string_hash(test1) != string_hash(test2));
        assert!(string_hash(test1) != string_hash(test3));
        assert!(string_hash(test1) != string_hash(test5));
        assert_eq!(string_hash(test1), string_hash(test4));
    }

    #[test]
    fn test_string_nocase_hash() {
        let mut test1 = cstr!("this is a test");
        let mut test2 = cstr!("this is a tesu");
        let mut test3 = cstr!("this is a test ");
        let mut test4 = cstr!("this is a test");
        let mut test5 = cstr!("This is a test");

        assert!(string_nocase_hash(test1) != string_nocase_hash(test2));
        assert!(string_nocase_hash(test1) != string_nocase_hash(test3));
        assert_eq!(string_nocase_hash(test1), string_nocase_hash(test5));
        assert_eq!(string_nocase_hash(test1), string_nocase_hash(test4));
    }

    #[test]
    fn test_pointer_hash() {
        let mut array: Array<i32, {num_test_values!()}> = Default::default();
        let mut i: i32;
        let mut j: i32;
        c_for!(i = 0; i < num_test_values!(); i += 1; {
            array[i] = 0;
        });
        c_for!(i = 0; i < num_test_values!(); i += 1; {
            c_for!(j = i + 1; j < num_test_values!(); j += 1; {
                assert!(pointer_hash(c_ref!(array[i]).cast()) != pointer_hash(c_ref!(array[j]).cast()));
            });
        });
    }

    #[test]
    fn test_int_hash() {
        let mut array: Array<i32, {num_test_values!()}> = Default::default();
        let mut i: i32;
        let mut j: i32;
        for i in 0..num_test_values!() {
            array[i] = i as i32;
        }
        for i in 0..num_test_values!() {
            for j in (i + 1)..num_test_values!() {
                assert!(int_hash(c_ref!(array[i]).cast()) != int_hash(c_ref!(array[j]).cast()));
            }
        }
        i = 5000;
        j = 5000;
        assert!(int_hash(c_ref!(i).cast()) == int_hash(c_ref!(j).cast()));
    }

    #[test]
    fn test_int_compare() {
        let mut a = 4;
        let mut b = 8;
        let mut c = 4;
        assert_eq!(int_compare(c_ref!(a), c_ref!(b)), -1);
        assert_eq!(int_compare(c_ref!(b), c_ref!(a)), 1);
        assert_eq!(int_compare(c_ref!(a), c_ref!(c)), 0);
    }
    
    #[test]
    fn test_int_equal() {
        let mut a = 4;
        let mut b = 8;
        let mut c = 4;
        assert_ne!(int_equal(c_ref!(a), c_ref!(c)), 0);
        assert_eq!(int_equal(c_ref!(a), c_ref!(b)), 0);
    }

    #[test]
    fn test_pointer_compare() {
        let mut array: Array<i32, 5> = Default::default();
        assert!(pointer_compare(c_ref!(array[0]).cast(), c_ref!(array[4]).cast()) < 0);
        assert!(pointer_compare(c_ref!(array[3]).cast(), c_ref!(array[2]).cast()) > 0);
        assert_eq!(pointer_compare(c_ref!(array[4]).cast(), c_ref!(array[4]).cast()), 0);
    }
    
    #[test]
    fn test_pointer_equal() {
        let mut a = 0;
        let mut b = 0;
        assert!(pointer_equal(c_ref!(a), c_ref!(a)) != 0);
        assert_eq!(pointer_equal(c_ref!(a), c_ref!(b)), 0);
    }

    #[test]
    fn test_string_compare() {
        let mut test1 = cstr!("Apple");
        let mut test2 = cstr!("Orange");
        let mut test3 = cstr!("Apple");
        assert!(string_compare(test1, test2) < 0);
        assert!(string_compare(test2, test1) > 0);
        assert_eq!(string_compare(test1, test3), 0);
    }

    #[test]
    fn test_string_equal() {
        let mut test1 = cstr!("this is a test string");
        let mut test2 = cstr!("this is a test string ");
        let mut test3 = cstr!("this is a test strin");
        let mut test4 = cstr!("this is a test strinG");
        let mut test5 = cstr!("this is a test string");
        assert!(string_equal(test1, test5) != 0);
        assert_eq!(string_equal(test1, test2), 0);
        assert_eq!(string_equal(test1, test3), 0);
        assert_eq!(string_equal(test1, test4), 0);
    }

    #[test]
    fn test_string_nocase_compare() {
        let mut test1 = cstr!("Apple");
        let mut test2 = cstr!("Orange");
        let mut test3 = cstr!("Apple");
        let mut test4 = cstr!("Alpha");
        let mut test5 = cstr!("bravo");
        let mut test6 = cstr!("Charlie");
        assert!(string_nocase_compare(test1, test2) < 0);
        assert!(string_nocase_compare(test2, test1) > 0);
        assert_eq!(string_nocase_compare(test1, test3), 0);
        assert!(string_nocase_compare(test4, test5) < 0);
        assert!(string_nocase_compare(test5, test6) < 0);
    }

    #[test]
    fn test_string_nocase_equal() {
        let mut test1 = cstr!("this is a test string");
        let mut test2 = cstr!("this is a test string ");
        let mut test3 = cstr!("this is a test strin");
        let mut test4 = cstr!("this is a test strinG");
        let mut test5 = cstr!("this is a test string");
        assert!(string_nocase_equal(test1, test5) != 0);
        assert_eq!(string_nocase_equal(test1, test2), 0);
        assert_eq!(string_nocase_equal(test1, test3), 0);
        assert!(string_nocase_equal(test1, test4) != 0);
    }
}