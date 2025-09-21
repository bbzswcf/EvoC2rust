pub fn binomial_tree_merge(mut heap: Ptr<BinomialHeap>, mut tree1: Ptr<BinomialTree>, mut tree2: Ptr<BinomialTree>) -> Ptr<BinomialTree> {
    let mut new_tree: Ptr<BinomialTree>;
    let mut tmp: Ptr<BinomialTree>;
    let mut i: i32 = Default::default();

    if (binomial_heap_cmp(heap, tree1.value, tree2.value) > 0) {
        tmp = tree1;
        tree1 = tree2;
        tree2 = tmp;
    }

    new_tree = c_malloc!(c_sizeof!(BinomialTree));

    if (new_tree == NULL!()) {
        return NULL!();
    }

    new_tree.refcount = 0;
    new_tree.order = (tree1.order + 1).cast::<u16>();

    new_tree.value = tree1.value;

    new_tree.subtrees = c_malloc!(c_sizeof!(Ptr<BinomialTree>) * new_tree.order);

    if (new_tree.subtrees == NULL!()) {
        c_free!(new_tree);
        return NULL!();
    }

    c_memcpy!(new_tree.subtrees, tree1.subtrees, c_sizeof!(Ptr<BinomialTree>) * tree1.order);
    let tmp0 = new_tree.order - 1;
    new_tree.subtrees[tmp0] = tree2;

    c_for!(let mut i = 0; i < new_tree.order; i.prefix_plus_plus(); {
        binomial_tree_ref(new_tree.subtrees[i]);
    });

    return new_tree;
}
