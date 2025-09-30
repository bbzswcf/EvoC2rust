## Case of post-generation repair

This document outlines a multi-stage repair process applied after the initial C-to-Rust code generation. The process includes bracket repair, rule-based repair, and LLM refinement.

---

### 1. Bracket Repair

LLM generation can sometimes produce code with mismatched brackets. This can interfere with subsequent automated, rule-based repair steps. Therefore, the first step after initial generation is to use an LLM to specifically identify and fix any bracket mismatches.

#### Initial code (with bracket error)

```rust
pub fn BzpBuffToBlockRLC(mut bzpf: Ptr<BzpFile>, mut bwt: Ptr<BzpBwtInfo>, mut IsLastdata: bool) {
    while (!BZP_BLOCK_FULL!(bwt).as_bool() && (!BZP_BUFF_READ_EMPTY!(bzpf)).as_bool() {
        let mut pos: i32 = bzpf.input.pos.cast();
        let mut ch: u8 = bzpf.input.buf[pos].cast::<u8>();
        let mut lasch: u8 = bzpf.lasChar.cast::<u8>();
        if (ch != lasch).as_bool() || (bzpf.num == BZP_RLC_NUM_UPPER_LIMIT!()).as_bool() {
            BzpAddCharToBlock(lasch.cast(), bzpf.num.cast(), bwt.cast());
            bzpf.lasChar = ch.cast();
            bzpf.num = 1;
        } else {
            bzpf.num += 1;
        }
        bzpf.input.pos += 1;
    }
    if IsLastdata.as_bool() && BZP_BUFF_READ_EMPTY!(bzpf).as_bool() {
        BzpAddCharToBlock(bzpf.lasChar.cast(), bzpf.num.cast(), bwt.cast());
        bzpf.lasChar = BZP_ASCII_SIZE!();
        bzpf.num = 0;
    }
}
```

#### Repaired Code

The LLM correctly adds the missing parenthesis to the `while` loop condition.

```rust
pub fn BzpBuffToBlockRLC(mut bzpf: Ptr<BzpFile>, mut bwt: Ptr<BzpBwtInfo>, mut IsLastdata: bool) {
    while (!BZP_BLOCK_FULL!(bwt).as_bool()).as_bool() && (!BZP_BUFF_READ_EMPTY!(bzpf).as_bool()).as_bool() {
        let mut pos: i32 = bzpf.input.pos.cast();
        let mut ch: u8 = bzpf.input.buf[pos].cast::<u8>();
        let mut lasch: u8 = bzpf.lasChar.cast::<u8>();
        if (ch != lasch).as_bool() || (bzpf.num == BZP_RLC_NUM_UPPER_LIMIT!()).as_bool() {
            BzpAddCharToBlock(lasch.cast(), bzpf.num.cast(), bwt.cast());
            bzpf.lasChar = ch.cast();
            bzpf.num = 1;
        } else {
            bzpf.num += 1;
        }
        bzpf.input.pos += 1;
    }
    if IsLastdata.as_bool() && BZP_BUFF_READ_EMPTY!(bzpf).as_bool() {
        BzpAddCharToBlock(bzpf.lasChar.cast(), bzpf.num.cast(), bwt.cast());
        bzpf.lasChar = BZP_ASCII_SIZE!();
        bzpf.num = 0;
    }
}
```

---

### 2. Rule-Based Repair

**Problem:** C allows implicit type conversions, whereas Rust requires explicit casting, making direct translation prone to type errors. LLMs often struggle with Rust's type inference rules, leading to either missing or unnecessary `.cast()` calls. For instance, Rust can infer types for assignments but not always for compound assignment operators like `+=`.

**Solution:** A rule-based approach is employed.
1.  The LLM is instructed to proactively add `.cast()` in any situation where a type conversion might be needed.
2.  A rule-based system then iteratively removes each `.cast()` and recompiles the code.
3.  If removing a cast does not increase the number of compiler errors, it is deemed unnecessary and the removal is kept. Otherwise, the cast is restored.

#### Original C code

```c
void BzpCalculateCost(BzpHuffmanGroups *huffman, int32_t st, int32_t ed)
{
    (void)memset_s(huffman->cost, sizeof(huffman->cost), 0, sizeof(huffman->cost));
    int32_t nGroups = huffman->nGroups;
    for (int32_t k = st; k <= ed; k++)
    {
        for (int32_t t = 0; t < nGroups; t++)
        {
            huffman->cost[t] += huffman->huffmanGroups[t].len[huffman->block[k]];
        }
    }
}
```

#### Initial code (with excessive casts)

```rust
pub fn BzpCalculateCost(mut huffman: Ptr<BzpHuffmanGroups>, mut st: i32, mut ed: i32) {
    c_memset_s!(huffman.cost, c_sizeofval!(huffman.cost), 0, c_sizeofval!(huffman.cost));
    cast::<Void>();
    let mut nGroups: i32 = huffman.nGroups.cast();
    c_for!(let mut k: i32 = st; k <= ed; k.suffix_plus_plus(); {
        c_for!(let mut t: i32 = 0; t < nGroups; t.suffix_plus_plus(); {
            huffman.cost[t] += huffman.huffmanGroups[t].len[huffman.block[k]].cast();
        });
    });
}
```

#### Rule-based repaired code

The rule-based system identifies and removes the redundant `.cast()` calls, resulting in cleaner, correct code.

```rust
pub fn BzpCalculateCost(mut huffman: Ptr<BzpHuffmanGroups>, mut st: i32, mut ed: i32) {
    c_memset_s!(huffman.cost, c_sizeofval!(huffman.cost), 0, c_sizeofval!(huffman.cost));
    cast::<Void>();
    let mut nGroups: i32 = huffman.nGroups;
    c_for!(let mut k: i32 = st; k <= ed; k.suffix_plus_plus(); {
        c_for!(let mut t: i32 = 0; t < nGroups; t.suffix_plus_plus(); {
            huffman.cost[t] += huffman.huffmanGroups[t].len[huffman.block[k]];
        });
    });
}
```

---

### 3. LLM Refinement

**Problem:** If rule-based repairs are insufficient to produce compiling code, the remaining errors are addressed by the LLM. The LLM is provided with the full context: the original C code, the current (still erroneous) Rust code, and the compiler's error messages.

**Process:** The LLM attempts a fix. If the fix reduces the number of compiler errors, it is accepted. This process can be repeated iteratively until the code compiles.

In this example, the compiler reported two main errors after the rule-based phase:
1.  The variable `ed` was used without being initialized.
2.  A borrow-checking error where `huffman` was borrowed as both mutable and immutable simultaneously.

The LLM successfully fixed both issues. However, this process can exhibit instability; for instance, the LLM unnecessarily removed a `.cast::<i32>()` call which was not causing an error.

#### Initial code (with remaining errors)

```rust
pub fn BzpHuffmanMain(mut huffman: Ptr<BzpHuffmanGroups>) {
    let mut nGroups: i32 = BzpGetHuffmanGroups(huffman.nBlock);
    huffman.nGroups = nGroups;
    BzpInitLenArray(huffman);
    let mut st: i32 = 0;
    let mut ed: i32;
    c_for!(let mut i: i32 = 0; i < BZP_MAX_ITER_NUM!(); i.suffix_plus_plus(); {
        c_for!(let mut j: i32 = 0; j < nGroups; j.suffix_plus_plus(); {
            c_memset_s!(huffman.huffmanGroups[j].weight, c_sizeofval!(huffman.huffmanGroups[j].weight), 0, c_sizeofval!(huffman.huffmanGroups[j].weight)).cast::<Void>();
        });
        st = 0;
        huffman.nSelect = 0;
        while (st < huffman.nBlock) {
            ed = BZP_MIN_FUN!(huffman.nBlock, st + BZP_ELEMS_NUM_IN_ONE_GROUP!()).cast::<i32>() - 1;
            BzpCalculateCost(huffman, st, ed);
            let mut id: i32 = BzpSelectTree(huffman);
            c_for!(let mut k: i32 = st; k <= ed; k.suffix_plus_plus(); {
                huffman.huffmanGroups[id].weight[huffman.block[k]] += 1;
            });
            st = ed + 1;
        }
        c_for!(let mut j: i32 = 0; j < nGroups; j.suffix_plus_plus(); {
            BzpBuildTreeBalanceHeight(c_ref!(huffman.huffmanGroups[j]));
        });
        BzpGenerateSelectMTF(huffman);
        c_for!(let mut i: i32 = 0; i < nGroups; i.suffix_plus_plus(); {
            BzpGetHuffmanTable(c_ref!(huffman.huffmanGroups[i]));
        });
    });
}
```

#### LLM repaired code

The LLM initializes `ed` and resolves the borrow-checking error by introducing a temporary variable `block_k`.

```rust
pub fn BzpHuffmanMain(mut huffman: Ptr<BzpHuffmanGroups>) {
    let mut nGroups: i32 = BzpGetHuffmanGroups(huffman.nBlock);
    huffman.nGroups = nGroups;
    BzpInitLenArray(huffman);
    let mut st: i32 = 0;
    let mut ed: i32 = Default::default();
    c_for!(let mut i: i32 = 0; i < BZP_MAX_ITER_NUM!(); i.suffix_plus_plus(); {
        c_for!(let mut j: i32 = 0; j < nGroups; j.suffix_plus_plus(); {
            c_memset_s!(huffman.huffmanGroups[j].weight, c_sizeofval!(huffman.huffmanGroups[j].weight), 0, c_sizeofval!(huffman.huffmanGroups[j].weight)).cast::<Void>();
        });
        st = 0;
        huffman.nSelect = 0;
        while (st < huffman.nBlock) {
            ed = BZP_MIN_FUN!(huffman.nBlock, st + BZP_ELEMS_NUM_IN_ONE_GROUP!()) - 1;
            BzpCalculateCost(huffman, st, ed);
            let mut id: i32 = BzpSelectTree(huffman);
            c_for!(let mut k: i32 = st; k <= ed; k.suffix_plus_plus(); {
                let block_k = huffman.block[k];
                huffman.huffmanGroups[id].weight[block_k] += 1;
            });
            st = ed + 1;
        }
        c_for!(let mut j: i32 = 0; j < nGroups; j.suffix_plus_plus(); {
            BzpBuildTreeBalanceHeight(c_ref!(huffman.huffmanGroups[j]));
        });
        BzpGenerateSelectMTF(huffman);
        c_for!(let mut i: i32 = 0; i < nGroups; i.suffix_plus_plus(); {
            BzpGetHuffmanTable(c_ref!(huffman.huffmanGroups[i]));
        });
    });
}
```