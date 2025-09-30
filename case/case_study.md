## Case study: Translating the `rb_tree_rotate` function

This case examines the translation of a C function for rotating a node in a red-black tree into Rust, comparing different baselines with our method EvoC2Rust.

### Original C Code

This is the C function to be translated. It performs a rotation on a given node within the tree.

```c
static RBTreeNode *rb_tree_rotate(RBTree *tree, RBTreeNode *node,
                                  RBTreeNodeSide direction)
{
    RBTreeNode *new_root;

    /* The child of this node will take its place:
       for a left rotation, it is the right child, and vice versa. */

    new_root = node->children[1-direction];

    /* Make new_root the root, update parent pointers. */

    rb_tree_node_replace(tree, node, new_root);

    /* Rearrange pointers */

    node->children[1-direction] = new_root->children[direction];
    new_root->children[direction] = node;

    /* Update parent references */

    node->parent = new_root;

    if (node->children[1-direction] != NULL) {
        node->children[1-direction]->parent = node;
    }

    return new_root;
}
```

---

### Baseline 1: Direct Prompting

This approach attempts a direct translation of the C code using a large language model.

**Rust Code:**
```rust
fn rb_tree_node_replace(tree: &mut RBTree, node1: &mut RBTreeNode, node2: Option<&mut RBTreeNode>) {
    //...
}

fn rb_tree_rotate(tree: &mut RBTree, node: *mut RBTreeNode, direction: RBTreeNodeSide) -> *mut RBTreeNode {
    let new_root = unsafe { (*node).children[1 - direction as usize] };

    rb_tree_node_replace(tree, node, new_root);

    unsafe {
        (*node).children[1 - direction as usize] = (*new_root).children[direction as usize];
        (*new_root).children[direction as usize] = node;
        (*node).parent = new_root;

        if !(*node).children[1 - direction as usize].is_null() {
            (*(*node).children[1 - direction as usize]).parent = node;
        }
    }

    new_root
}
```

**Compiler Error:**
```
error[E0308]: arguments to this function are incorrect
  --> src/src/rb_tree_c.rs:67:5
   |
67 |     rb_tree_node_replace(tree, node, new_root);
   |     ^^^^^^^^^^^^^^^^^^^^
   |
note: expected `&mut RBTreeNode`, found `*mut RBTreeNode`
  --> src/src/rb_tree_c.rs:67:32
   |
67 |     rb_tree_node_replace(tree, node, new_root);
   |                                ^^^^
   = note: expected mutable reference `&mut rb_tree_c::RBTreeNode`
                    found raw pointer `*mut rb_tree_c::RBTreeNode`
note: expected `Option<&mut RBTreeNode>`, found `*mut RBTreeNode`
  --> src/src/rb_tree_c.rs:67:38
   |
67 |     rb_tree_node_replace(tree, node, new_root);
   |                                      ^^^^^^^^
   = note:     expected enum `Option<&mut rb_tree_c::RBTreeNode>`
           found raw pointer `*mut rb_tree_c::RBTreeNode`
note: function defined here
  --> src/src/rb_tree_c.rs:46:4
   |
46 | fn rb_tree_node_replace(tree: &mut RBTree, node1: &mut RBTreeNode, node2: Option<&mut RBTreeNode>) {
   |    ^^^^^^^^^^^^^^^^^^^^                    ----------------------  ------------------------------
```

**Analysis:**
The translation fails due to a project-level error from mismatched cross-function interfaces. The generated code tries to pass raw pointers (`*mut RBTreeNode`) to the `rb_tree_node_replace` function, which expects mutable references (`&mut RBTreeNode` and `Option<&mut RBTreeNode>`).

---

### Baseline 2: Self-Repair

This approach attempts to write idiomatic Rust code using ownership and borrowing rules, followed by an attempt to fix the resulting compiler errors.

**Rust Code:**
```rust
impl RBTree {
//...
    fn rotate(&mut self, node: &mut RBTreeNode, direction: RBTreeNodeSide) -> Option<Box<RBTreeNode>> {
        let opposite = match direction {
            RBTreeNodeSide::Left => RBTreeNodeSide::Right,
            RBTreeNodeSide::Right => RBTreeNodeSide::Left,
        };

        if let Some(mut new_root) = node.children[opposite as usize].take() {
            self.node_replace(node, Some(new_root.clone()));

            node.children[opposite as usize] = new_root.children[direction as usize].take();
            if let Some(ref mut child) = node.children[opposite as usize] {
                child.parent = Some(Box::new((*node).clone()));
            }

            new_root.children[direction as usize] = Some(Box::new((*node).clone()));
            node.parent = Some(new_root); // move occurs here

            Some(new_root)  // value used after move
        } else {
            None
        }
    }
//...
}
```

**Compiler Error:**
```
error[E0382]: use of moved value: `new_root`
   --> src/rb_tree.rs:112:18
    |
101 |         if let Some(mut new_root) = node.children[opposite as usize].take() {
    |                     ------------ move occurs because `new_root` has type `Box<RBTreeNode>`, which does not implement the `Copy` trait
...
110 |             node.parent = Some(new_root);
    |                                -------- value moved here
111 |
112 |             Some(new_root)
    |                  ^^^^^^^^ value used here after move
    |
help: consider cloning the value if the performance cost is acceptable
    |
110 |             node.parent = Some(new_root.clone());
    |                                        ++++++++
```

**Analysis:**
The code runs into an ownership transfer error within a single function. The `new_root` value is moved when it's assigned to `node.parent`, and then it's used again in the return statement, which is not allowed by Rust's ownership rules.

---

### Our mothod: EvoC2Rust

**Rust Code:**
```rust
pub fn rb_tree_rotate(mut tree: Ptr<RBTree>, mut node: Ptr<RBTreeNode>, mut direction: RBTreeNodeSide) -> Ptr<RBTreeNode> {
    let mut new_root: Ptr<RBTreeNode>;
    new_root = node.children[1 - direction].cast();
    rb_tree_node_replace(tree.cast(), node.cast(), new_root.cast());
    // cast() ensures interface consistency
    node.children[1 - direction] = new_root.children[direction].cast();
    new_root.children[direction] = node.cast();
    node.parent = new_root.cast();
    if (node.children[1 - direction] != NULL!()).as_bool() {
        node.children[1 - direction].parent = node.cast();
    }
    return new_root.cast();
}
```

**Analysis:**
- **Project-level consistency:** EvoC2Rust employs a unified `Ptr<T>` type system to ensure compatible function interfaces across the project.
- **Function-level correctness:** EvoC2Rust directly maps C pointer operations to the `Ptr<T>` type, avoiding the complex ownership and borrowing issues encountered in the idiomatic Rust approach. This results in a correct and compilable translation.