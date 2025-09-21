pub fn binomial_heap_merge(mut heap: Ptr<BinomialHeap>, mut other: Ptr<BinomialHeap>) -> i32 {
    let mut new_roots: Ptr<Ptr<BinomialTree>>;
    let mut new_roots_length: u32;
    let mut vals: Array<Ptr<BinomialTree>, 3> = Default::default();
    let mut num_vals: i32;
    let mut carry: Ptr<BinomialTree> = Default::default();
    let mut new_carry: Ptr<BinomialTree> = Default::default();
    let mut max: u32;
    let mut i: u32;

    if (heap.roots_length > other.roots_length).as_bool() {
        max = (heap.roots_length + 1).cast();
    } else {
        max = (other.roots_length + 1).cast();
    }

    new_roots = c_malloc!(c_sizeof!(Ptr<BinomialTree>) * max);

    if (new_roots == NULL!()).as_bool() {
        return 0;
    }

    new_roots_length = 0;
    carry = NULL!();

    c_for!(i = 0; i < max; i.prefix_plus_plus(); {
        num_vals = 0;

        if (i < heap.roots_length && heap.roots[i] != NULL!()).as_bool() {
            vals[num_vals] = heap.roots[i].cast();
            num_vals += 1;
        }

        if (i < other.roots_length && other.roots[i] != NULL!()).as_bool() {
            vals[num_vals] = other.roots[i].cast();
            num_vals += 1;
        }

        if (carry != NULL!()).as_bool() {
            vals[num_vals] = carry.cast();
            num_vals += 1;
        }

        if ((num_vals & 1) != 0).as_bool() {
            new_roots[i] = vals[num_vals - 1].cast();
            binomial_tree_ref(new_roots[i].cast());
            new_roots_length = (i + 1).cast();
        } else {
            new_roots[i] = NULL!();
        }

        if ((num_vals & 2) != 0).as_bool() {
            new_carry = binomial_tree_merge(heap.cast(), vals[0].cast(), vals[1].cast());

            if (new_carry == NULL!()).as_bool() {
                binomial_heap_merge_undo(new_roots.cast(), i.cast());
                binomial_tree_unref(carry.cast());
                return 0;
            }
        } else {
            new_carry = NULL!();
        }

        binomial_tree_unref(carry.cast());
        carry = new_carry.cast();
        binomial_tree_ref(carry.cast());
    });

    c_for!(i = 0; i < heap.roots_length; i.prefix_plus_plus(); {
        if (heap.roots[i] != NULL!()).as_bool() {
            binomial_tree_unref(heap.roots[i].cast());
        }
    });

    c_free!(heap.roots);
    heap.roots = new_roots.cast();
    heap.roots_length = new_roots_length.cast();

    return 1;
}
