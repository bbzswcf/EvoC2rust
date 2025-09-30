definition_prompt = """\
Translate the C Code to Rust. 
You need to translate the definition only.
Notice that: 
You need to translate `void*` type to VoidPtr in Rust, and all char type to u8.
Array in C like int[10] should be translated to `Array` type in Rust: Array<i32, 10>, and you should use arr! macro to initialize the array, for example, `int a[5] = {1, 2, 3, 4, 5};` should be translated to `a: Array<i32, 5> = arr![1, 2, 3, 4, 5];`.
Enum Type in C should be translated to i32 and the enum values should be translated to macro_rules, and all translated macros in Rust should be uppercased.
Pointers in C should be translated to Ptr<T> in Rust.
Remember when translating macros, add `pub(crate)` to the macro definition to make it visible.
You should translate the global variables start with g_ with Global<T> type and global!() macro, for example, `static int g_a = 0;` should be translated to `pub static g_a: Global<i32> = global!(0);`. However, if it not not start with g_, just translate it to a constant.

Here are some examples:
Source:
```c
typedef void *MY_VALUE;
```
Translation:
```rust
pub type MyValue = VoidPtr;
```
Source:
```
#define MY_NULL 0
```
Translation:
```rust
macro_rules! MY_NULL { () => { NULL!() } }
pub(crate) use MY_NULL;
```

Source:
```c
typedef struct _MyStruct MyStruct;
```

Translation:
```rust
pub type MyStruct = _MyStruct;
```

Source:
```c
typedef enum
{
    MY_RED = 0,
    MY_GREEN,
    MY_BLUE
} MyEnum;
```

Translation:
```rust
pub type MyEnum = i32;
macro_rules! MY_RED { () => { 0 } }
pub(crate) use MY_RED;
macro_rules! MY_GREEN { () => { 1 } }
pub(crate) use MY_GREEN;
macro_rules! MY_BLUE { () => { 2 } }
pub(crate) use MY_BLUE;
```

Source:
```c
typedef int (*MyFunction)(int a, int b);
```

Translation:
```rust
pub type MyFunction = FuncPtr<fn(i32, i32) -> i32>;
```

Source:
```c
typedef void (*ANO_function)(const void* a, char* b);
```

Translation:
```rust
pub type ANO_function = FuncPtr<fn(VoidPtr, Ptr<u8>)>;
```

Source:
```c
static MyFunction g_MyCustomFunc = NULL;
```

Translation:
```rust
pub static g_MyCustomFunc: Global<MyFunction> = global!(NULL!());
```

When translating string literals in C, use cstr! macro

Source:
```c
const char* g_MyGlobalStr = "Hello, World!";
```

Translation:
```rust
pub static g_MyGlobalStr: Global<Ptr<u8>> = global!(cstr!("Hello, World!"));
```

Source:
```c
static MyFunction g_MyCustomFunc = NULL;
```


Source:
```c
int[] g_MyCustomArray = {1, 2, 3, 4, 5};
const int[] myCustomArray = {1, 2, 3, 4, 5};
```

Translation:
```rust
pub static g_MyCustomArray: Global<Array<i32, 5>> = global!(arr![1, 2, 3, 4, 5]);
pub const myCustomArray: Array<i32, 5>> = arr![1, 2, 3, 4, 5];
```

Source:
```c
static int[] g_A10 = {3, 4, 5, 6, 7, 8, 9, 10, 11, 12};
static const int[] A10 = {3, 4, 5, 6, 7, 8, 9, 10, 11, 12};
```

Translation:
```
pub static g_A10: Global<Array<i32, 10>> = global!(arr![3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
pub const A10: Array<i32, 10> = arr![3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
```

Source:
```c
static const int arr_counts = sizeof(arr) / sizeof(int);
```

Translation:
```
pub const arr_counts: i32 = arr.len() as i32;
```

Source:
```c
struct MySimpleStruct {
    int* arr;
    unsigned int length;
};
```

Translation:
```rust
#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct MySimpleStruct {
    pub arr: Ptr<i32>,
    pub length: u32,
}
```

If a struct is defined with `typedef struct PreviousName { ... } PostName;`, add an additional type alias in Rust Translation: `// some declarations\npub struct PreviousName { ... }\n pub type PostName = PreviousName;`. If no `PreviousName` provided, like `typedef struct { ... } PostName;`, just translate with `// some declarations\npub struct PostName { ... }`.

Source:
```c
typedef struct _MyComplexStruct {
    MyStructEntry **vEntries;
    const char* vlength;
    MyStructValueFunc valueFunc;
    int values[64];
    FILE* file;
	MyStructNode *children[CHINDREN_SIZE];
    MyStructNode more_children[CHINDREN_SIZE * 5 + 1];
} MY_Com_Struct;
```

Translation:
```rust
#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct _MyComplexStruct {
    pub vEntries: Ptr<Ptr<MyStructEntry>>,
    pub vlength: Ptr<u8>,
    pub valueFunc: MyStructValueFunc,
    pub values: Array<i32, 64>,
    pub file: FilePtr,
    pub children: Array<Ptr<MyStructNode>, { CHINDREN_SIZE!() }>,
    pub more_children: Array<MyStructNode, { CHINDREN_SIZE!() * 5 + 1 }>,
}

pub type MY_Com_Struct = _MyComplexStruct;
```

Source:
```c
typedef struct _MySimpleStruct {
    int arr[2];
    unsigned int length;
    MySimpleStruct* ss; 
};
```

Translation:
```rust
#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct _MySimpleStruct {
    pub arr: Array<i32, 2>,
    pub length: u32,
    pub ss: Ptr<MySimpleStruct>,
}
```
"""

dummy_function_prompt = """\
Translate the C Code to Rust. 
You need to translate the function to a dummy function with unimplemented!() macro only.
Here are some rules you need to follow:
Type translation: 
    1. Basic types like int, char, unsigned char, uint32_t, etc. should be translated to Rust types: int -> i32, char -> u8, unsigned char -> u8, uint32_t -> u32, etc. 
    2. Pointers in C should be translated to Ptr<T> in Rust, and void* should be translated to Ptr<Void>. char and unsigned char should all be translated to u8, so char* should be translated to Ptr<u8>.
    3. If function has array parameters, translate it to a Ptr<T> type in Rust. For example, `void MyFunction(int a[5])` should be translated to `pub fn MyFunction(mut a: Ptr<i32>)`.
    4. FILE* in C should be translated to FilePtr type in Rust.
Now follow these examples for translation:

Source:
```c
void VOS_MD5CalcEx(char *output, uint32_t outputLen, const uint8_t *input, uint32_t inputLen)
{
    MD5_CTX context;
    if (outputLen < MD5_DIGEST_LEN)
    {
        return;
    }
    VOS_MD5Init(&context);
    VOS_MD5Update(&context, (uint8_t *)(uintptr_t)input, inputLen);
    VOS_MD5FinalEx(output, outputLen, &context);
}
```

Translation:
```rust
pub fn VOS_MD5CalcEx(mut output: Ptr<u8>, mut outputLen: u32, mut input: Ptr<u8>, mut inputLen: u32) {
    unimplemented!();
}
```

Source:
```c
void VosAvlRebalance(AVLBASE_NODE_S **ppstSubTree)
{
    int iMoment;
    iMoment = (*ppstSubTree)->sRHeight - (*ppstSubTree)->sLHeight;
    if (iMoment > 1)
    {
        if ((*ppstSubTree)->pstRight->sLHeight > (*ppstSubTree)->pstRight->sRHeight)
        {
            VosAvlRotateRight(&(*ppstSubTree)->pstRight);
        }
        VosAvlRotateLeft(ppstSubTree);
    }
    else if (iMoment < -1)
    {
        if ((*ppstSubTree)->pstLeft->sRHeight > (*ppstSubTree)->pstLeft->sLHeight)
        {
            VosAvlRotateLeft(&(*ppstSubTree)->pstLeft);
        }
        VosAvlRotateRight(ppstSubTree);
    }
    return;
}
```

Translation:
```rust
pub fn VosAvlRebalance(mut ppstSubTree: Ptr<Ptr<AVLBASE_NODE_S>>) {
    unimplemented!();
}
```

Source:
```c
static void RapidlzCopyMatchFast(uint8_t *dst, uint8_t *match, uint16_t offset, uint32_t length)
{
    uint8_t *dstCurr = dst;
    uint8_t *matchPtr = match;
    if (offset >= RAPIDLZ_SIXTEEN_BYTE)
    {
        RapidlzCopyLiteralsFast(matchPtr, dstCurr, length);
        return;
    }
    for (int i = 0; i < RAPIDLZ_EIGHT_BYTE; ++i)
    {
        dstCurr[i] = matchPtr[i];
    }
    if (length <= RAPIDLZ_EIGHT_BYTE)
    {
        return;
    }
    uint8_t *dstEnd = dstCurr + length;
    if (offset < RAPIDLZ_EIGHT_BYTE)
    {
        matchPtr += g_overlapOffAddVal[offset];
        dstCurr += RAPIDLZ_EIGHT_BYTE;
    }
    do
    {
        RapidlzCopy8Byte(dstCurr, matchPtr);
        dstCurr += RAPIDLZ_EIGHT_BYTE;
        matchPtr += RAPIDLZ_EIGHT_BYTE;
    } while (dstCurr < dstEnd);
}
```

Translation:
```rust
pub fn RapidlzCopyMatchFast(mut dst: Ptr<u8>, mut r#match: Ptr<u8>, mut offset: u16, mut length: u32) {
    unimplemented!();
}
```

Source:
```c
static uint16_t RapidlzReadLE16Bit(const void *addr)
{
    if (RapidlzIsLE() != 0)
    {
        return *(const uint16_t *)addr;
    }
    uint8_t tmp1 = ((const uint8_t *)addr)[0];
    uint8_t tmp2 = ((const uint8_t *)addr)[1];
    return (uint16_t)(tmp1 + (tmp2 << 8));
}
```

Translation:
```rust
pub fn RapidlzReadLE16Bit(mut addr: Ptr<Void>) -> u16 {
    unimplemented!();
}
```

Source:
```c
void *VOS_AVL_Find(AVL_TREE *pstTree, const void *pKey)
{
    AVL_NODE *pstNode;
    int iResult;
    if (pstTree == AVL_NULL_PTR)
    {
        return AVL_NULL_PTR;
    }
    pstNode = pstTree->pstRoot;
    while (pstNode != AVL_NULL_PTR)
    {
        iResult = pstTree->pfnCompare(pKey, pstNode->pKey);
        if (iResult > 0)
        {
            pstNode = pstNode->pstRight;
        }
        else if (iResult < 0)
        {
            pstNode = pstNode->pstLeft;
        }
        else
        {
            break;
        }
    }
    return ((pstNode != AVL_NULL_PTR) ? pstNode->pSelf : AVL_NULL_PTR);
}
```

Translation:
```rust
pub fn VOS_AVL_Find(mut pstTree: Ptr<AVL_TREE>, mut pKey: Ptr<Void>) -> Ptr<Void> {
    unimplemented!();
}
```

Source:
```c
uint32_t BzpReadUInt24(InDeComdata *inData)
{
    uint8_t ch;
    uint32_t val = 0;
    ch = BzpReadBits(BZP_BITS8, inData);
    val = (val << BZP_BITS8) | ((uint32_t)ch);
    ch = BzpReadBits(BZP_BITS8, inData);
    val = (val << BZP_BITS8) | ((uint32_t)ch);
    ch = BzpReadBits(BZP_BITS8, inData);
    val = (val << BZP_BITS8) | ((uint32_t)ch);
    return val;
}
```

Translation:
```rust
pub fn BzpReadUInt24(mut inData: Ptr<InDeComdata>) -> u32 {
    unimplemented!();
}
```

Source:
```c
int32_t BzpGetDictionaryList(InDeComdata *inData)
{
    int32_t ninUse = 0;
    bool use16[16] = {0};
    bool inUse[BZP_ASCII_SIZE] = {0};
    for (int32_t i = 0; i < BZP_GROUPS_ASCII; i++)
    {
        use16[i] = BzpReadBits(BZP_BIT, inData);
    }
    for (int32_t i = 0; i < BZP_GROUPS_ASCII; i++)
    {
        if (use16[i])
        {
            for (int32_t j = 0; j < BZP_CHARS_PER_GROUP_ASCII; j++)
            {
                inUse[i * BZP_GROUPS_ASCII + j] = BzpReadBits(BZP_BIT, inData);
            }
        }
    }
    for (int32_t i = 0; i < BZP_ASCII_SIZE; i++)
    {
        if (inUse[i])
        {
            inData->list[ninUse++] = i;
        }
    }
    return ninUse;
}
```

Translation:
```rust
pub fn BzpGetDictionaryList(mut inData: Ptr<InDeComdata>) -> i32 {
    unimplemented!();
}
```

Source:
```c
void CmptlzLogWrite(size_t errorCode, const char *funcName, unsigned short line, const char *fmt, ...)
{
    va_list alist;
    char output[LOG_BUF_SIZE];
    int ret;
    size_t len;
    CmptlzLogFunc func = g_cmptlzLogFunc;
    if (func == NULL)
    {
        return;
    }
    ret = snprintf_s(output, LOG_BUF_SIZE, LOG_BUF_SIZE - 1, "\\n[Cmptlz-Log] Func=%s, Line=%u, Error=0x%zx\\n", funcName,
                     line, errorCode);
    if (ret < 0)
    {
        return;
    }
    len = (size_t)ret;
    va_start(alist, fmt);
    ret = vsnprintf_s(output + len, LOG_BUF_SIZE - len, LOG_BUF_SIZE - len - 1, fmt, alist);
    va_end(alist);
    if (ret < 0)
    {
        return;
    }
    func(output, strlen(output) + 1);
}
```

Translation:
```rust
pub fn CmptlzLogWrite(mut errorCode: usize, mut funcName: Ptr<u8>, mut line: u16, mut fmt: Ptr<u8>, mut alist: VaList) {
    unimplemented!();
}
```

Source:
```c
int CmptLzDecSinglePacket(CmptLzDecCtx *decCtx, size_t dicPosLimit, const unsigned char *pSrcIn, size_t srcInLen,
                          size_t *psrcCostLen)
{
    int res;
    size_t lookAheadLen = 0;
    uint32_t newTempBufSize = decCtx->tempBufSize;
    unsigned char *oldTmpBuf = &(decCtx->tempBuf[decCtx->tempBufSize]);
    while (newTempBufSize < CMPTLZ_REQUIRED_INPUT_MAX && lookAheadLen < srcInLen)
    {
        decCtx->tempBuf[newTempBufSize++] = pSrcIn[lookAheadLen++];
    }
    const unsigned char *bufLimit = decCtx->tempBuf + newTempBufSize;
    res = CmptLzTryDecOnePacket(decCtx, &(decCtx->tempBuf[0]), &bufLimit);
    if (res == CMPTLZ_DEC_INPUT_EOF)
    {
        *psrcCostLen = lookAheadLen;
        decCtx->tempBufSize = newTempBufSize;
        return CMPTLZ_DEC_INPUT_EOF;
    }
    if (res == CMPT_ERROR_DATA)
    {
        return res;
    }
    decCtx->buf = &(decCtx->tempBuf[0]);
    res = CmptLzDecDirectProcess(decCtx, dicPosLimit, bufLimit);
    if ((res != CMPT_OK) || (bufLimit != decCtx->buf) || (bufLimit <= oldTmpBuf))
    {
        *psrcCostLen = 0;
        return CMPT_ERROR_DATA;
    }
    *psrcCostLen = (size_t)(bufLimit - oldTmpBuf);
    decCtx->tempBufSize = 0;
    return res;
}
```

Translation:
```rust
pub fn CmptLzDecSinglePacket(mut decCtx: Ptr<CmptLzDecCtx>, mut dicPosLimit: usize, mut pSrcIn: Ptr<u8>, mut srcInLen: usize, mut psrcCostLen: Ptr<usize>) -> i32 {
    unimplemented!();
}
```

Source:
```c
BzpHuffmanDecode *BzpHuffmanDecodeInit(int32_t blockSize)
{
    if (BZP_INVALID_BLOCK_SIZE(blockSize))
    {
        return NULL;
    }
    BzpHuffmanDecode *huffman = (BzpHuffmanDecode *)malloc(sizeof(BzpHuffmanDecode));
    if (huffman == NULL)
    {
        return NULL;
    }
    int32_t spaceSize = BZP_BASE_BLOCK_SIZE * blockSize / BZP_ELEMS_NUM_IN_ONE_GROUP;
    huffman->select = (int32_t *)malloc(spaceSize * sizeof(int32_t));
    if (huffman->select == NULL)
    {
        BzpHuffmanDecodeFinish(huffman);
    }
    (void)memset_s(huffman->base, sizeof(huffman->base), 0, sizeof(huffman->base));
    (void)memset_s(huffman->perm, sizeof(huffman->perm), 0, sizeof(huffman->perm));
    (void)memset_s(huffman->limit, sizeof(huffman->limit), 0, sizeof(huffman->limit));
    huffman->selectCnt = 0;
    huffman->deCodeNum = 0;
    return huffman;
}
```

Translation:
```rust
pub fn BzpHuffmanDecodeInit(mut blockSize: i32) -> Ptr<BzpHuffmanDecode> {
    unimplemented!();
}
```
"""

function_prompt = """\
Translate the C Code to Rust. 
You need to translate the function only.
Here are some rules you need to follow:
Type translation: 
    1. Basic types like int, char, unsigned char, uint32_t, etc. should be translated to Rust types: int -> i32, char -> u8, unsigned char -> u8, uint32_t -> u32, etc. 
    2. Pointers in C should be translated to Ptr<T> in Rust, and void* should be translated to Ptr<Void>. char and unsigned char should all be translated to u8, so char* should be translated to Ptr<u8>.
    3. Array in C like int[10] should be translated to `Array` type in Rust: Array<i32, 10>, and you should use arr! macro to initialize the array, for example, `int a[5] = {1, 2, 3, 4, 5};` should be translated to `a: Array<i32, 5> = arr![1, 2, 3, 4, 5];`. Notice that if function has array parameters, translate it to a Ptr<T> type in Rust. For example, `void MyFunction(int a[5])` should be translated to `pub fn MyFunction(mut a: Ptr<i32>)`.
    4. FILE* in C should be translated to FilePtr type in Rust. You can use c_fread!(), f_fwrite!(), c_fopen!(), c_fclose!(), etc. to operate the file.
Operators:
    1. Always use the same operators in C which is available in Rust, like `+`, `-`, `*`, `+=` and `-=`, do not use methods like `.add()` or `.offset()` for pointers. The C `->` operator should be translated to `.` in Rust. For example, `a->b` should be translated to `a.b`.
    2. The `++` and `--` operators are not available in Rust, use .suffix_plus_plus(), .suffix_minus_minus() for suffix increment and decrement, and prefix_plus_plus() and prefix_minus_minus() for prefix increment and decrement. For example, `a++` -> `a.suffix_plus_plus()`, `++a` -> `a.prefix_plus_plus()`. 
    3. The `&` operator has different meaning in Rust, use macro `c_ref!()` instead. For example, `(void*)(&b)` should be translated to `c_ref!(b).cast::<Ptr<Void>>();`.
    4. The `sizeof` operator should be translated to Rust macro `c_sizeof!` for types and `c_sizeofval!` for variables. For example, `sizeof(int)` should be translated to `c_sizeof!(int)`, and `sizeof(my_struct->a)` should be translated to `c_sizeofval!(my_struct.a)`.
Macros vs Functions:
    1. Macros in C should be translated to Rust macros with the same name, also uppercased. For example, `a = MY_MACRO;` should be translated to `a = MY_MACRO!();`, and `a = MY_MACRO(b);` should be translated to `a = MY_MACRO!(b);`. Macros in C should ONLY CONTAIN uppercase letter, digits and underscores, like `MY_MACRO_NUM_2`. Anything contains lowercase letter is a funtion. Also, if some variable with only uppercase letter and digits is indexed, like `K256[a]`, it is not a macro, but a global array. Thus, `int a = K256[1];` should be translated to `let mut a: i32 = K256[1];`, don't use `K256!(1)`.
    2. Non C-builtin Functions in C should be translated to Rust functions with the same name. For example, `a = MY_Func(b);` should be translated to `a = MY_Func(b.cast()).cast();`. C-builtin Functions, like `malloc`, `free`, `strcmp`, `memmove_s`, `memcpy_s`, etc., should be translated to Rust macros with the same name, like `c_malloc!()`, `c_free!()`, `c_strcmp!()`, `c_memmove_s!()`, `c_memcpy_s!()`, etc. For example, `int* a = (int*)malloc(10 * sizeof(int));` should be translated to `let mut a: Ptr<i32> = c_malloc!(10 * c_sizeof!(int));`. They are macros, so don't use .cast() when pass parameters to them or make assignments.
Type casting:
    1. C have implicit type casting and explicit type casting. In Rust, you should use `.cast::<T>()` method for explicit type casting, and use `.cast()` for implicit type casting. 
    2. For example of explicit type casting, `int a = (int)b;` should be translated to `let mut a: i32 = b.cast::<i32>();`, and `b = (int32_t*)((char*)p + 8);` should be translated to `b = (p.cast::<Ptr<u8>>() + 8).cast::<Ptr<i32>>();`. 
    3. For implicit type casting, it exists in three conditions: assignments, function parameters, and return values. For example, `MyFunction(a, b)` should be translated to `MyFunction(a.cast(), b.cast());`, `a = b` should be translated to `a = b.cast();`, and `return a` should be translated to `return a.cast();`. Notice that only pass parameter for functions and assignments need to use .cast() method, for macros, do not use .cast() method. For example, `return MY_MACRO(a, b);` should be translated to `return MY_MACRO!(a, b);`, and `int a = MY_MACRO(b);` should be translated to `let mut a: i32 = MY_MACRO!(b);`.
    4. However, if the passed expression is not a typed value, which means: number literal(like `0` or `0x100`), string literal(like cstr!("abcd")), non-operator macro return value(like `MY_MACRO(a)`) do not need to be casted. For example. `MyFunction(0, a, 1 + MY_NUM, &expr, "abcd", MY_MACRO(b))` should be translated to `MyFunction(0, a.cast(), 1 + MY_NUM!(), c_ref!(expr).cast(), cstr!("abcd").cast(), MY_MACRO!(b));`. c_ref!(), c_sizeof!() and c_sizeofval!() are operator macros, so they should be casted. For example, `MyFunc(sizeof(int), sizeof(st.v), &st)` should be translated to `MyFunc(c_sizeof!(int).cast(), c_sizeofval!(st.v).cast(), c_ref!(st).cast());`.
Logical options:
    1. `for` is not available in Rust, use `c_for!` macro instead. For example, `for (int i = 0; i < 10; i++) {}` should be translated to `c_for!(let mut i = 0; i < 10; i.suffix_plus_plus(); {} );`. Those three parameters in `c_for!` can be empty, like `for(; i < 10;) {}` should be translated to `c_for!(; i < 10; {});`.
    2. `for` with no condition is not available in Rust, use `loop` instead. For example, `for(;;) {}` should be translated to `loop {}`.
    3. `while` is available in Rust, use `while` for `while` loop. For example, `while (a < 10) {}` should be translated to `while a < 10 {}`.
    4. `do while` is not available in Rust, use `c_do!` macro instead. For example, `do { a++; } while (a < 10);` should be translated to `c_do!({ a.suffix_plus_plus(); } while a < 10);`.
    5. `switch` is not available in Rust, use `if` and `else if` instead. For example, `switch(a) { case 1: i |= 1; case 2: i |= 2; break; default: i |= 4; break; }` should be translated to `c_switch!(a, { 1 => { i |= 1; break; }, 2 => { i |= 2; break; }, _ => { i |= 4; break; }, });`.
    6. Ternary operator `? :` is not available in Rust, use `if else` instead. For example, `a = (b > 0) ? 1 : 0;` should be translated to `a = if b > 0 { 1 } else { 0 };`.
    7. Notice that if the condition expression is not a comparison expression('==', '!=', '<', '>', '<=' and '>='), like `a = (b) ? 1 : 0;`, you should use `as_bool()` method to convert it to a boolean value. For example, `a = (b) ? 1 : 0;` shoule be translated to `a = if b.as_bool() { 1 } else { 0 };`.
Others:
    1. Notice that using one field for indexing another field in a struct cause compilation error in Rust. For example, `my_struct.arr[my_struct.field] = 1;` should be translated to `let idx: usize = my_struct.field.cast(); my_struct.arr[idx] = 1;`.
    2. Don't use uninitialized variables in Rust, use Default::default() to initialize the variable. For example, `int a;` should be translated to `let mut a: i32 = Default::default();`.
    3. When translating a global variable starts with `g_`, like `g_Offset`, use `.lock()`. For example, `int a = g_offsetCc[n];` should be translated to  `let a: i32 = (*g_offsetCc.lock())[n];`. However, global variable not starts with `g_` should not add `lock()`.
Now follow these examples for translation:

Source:
```c
void VOS_MD5CalcEx(uint8_t *output, uint32_t outputLen, const uint8_t *input, uint32_t inputLen)
{
    MD5_CTX context;
    if (outputLen < MD5_DIGEST_LEN)
    {
        return;
    }
    VOS_MD5Init(&context);
    VOS_MD5Update(&context, (uint8_t *)(uintptr_t)input, inputLen);
    VOS_MD5FinalEx(output, outputLen, &context);
}
```

Translation:
```rust
pub fn VOS_MD5CalcEx(mut output: Ptr<u8>, mut outputLen: u32, mut input: Ptr<u8>, mut inputLen: u32) {
    let mut context: MD5_CTX = Default::default();
    if (outputLen < MD5_DIGEST_LEN!()).as_bool() {
        return;
    }
    VOS_MD5Init(c_ref!(context).cast());
    VOS_MD5Update(c_ref!(context).cast(), input.cast::<Ptr<u8>>(), inputLen.cast());
    VOS_MD5FinalEx(output.cast(), outputLen.cast(), c_ref!(context).cast());
}
```

Source:
```c
void VosAvlRebalance(AVLBASE_NODE_S **ppstSubTree)
{
    int iMoment;
    iMoment = (*ppstSubTree)->sRHeight - (*ppstSubTree)->sLHeight;
    if (iMoment > 1)
    {
        if ((*ppstSubTree)->pstRight->sLHeight > (*ppstSubTree)->pstRight->sRHeight)
        {
            VosAvlRotateRight(&(*ppstSubTree)->pstRight);
        }
        VosAvlRotateLeft(ppstSubTree);
    }
    else if (iMoment < -1)
    {
        if ((*ppstSubTree)->pstLeft->sRHeight > (*ppstSubTree)->pstLeft->sLHeight)
        {
            VosAvlRotateLeft(&(*ppstSubTree)->pstLeft);
        }
        VosAvlRotateRight(ppstSubTree);
    }
    return;
}
```

Translation:
```rust
pub fn VosAvlRebalance(mut ppstSubTree: Ptr<Ptr<AVLBASE_NODE_S>>) {
    let mut iMoment: i32;
    iMoment = ((*ppstSubTree).sRHeight - (*ppstSubTree).sLHeight).cast();
    if (iMoment > 1).as_bool() {
        if ((*ppstSubTree).pstRight.sLHeight > (*ppstSubTree).pstRight.sRHeight).as_bool() {
            VosAvlRotateRight(c_ref!((*ppstSubTree).pstRight).cast());
        }
        VosAvlRotateLeft(ppstSubTree.cast());
    } else if (iMoment < -1).as_bool() {
        if ((*ppstSubTree).pstLeft.sRHeight > (*ppstSubTree).pstLeft.sLHeight).as_bool() {
            VosAvlRotateLeft(c_ref!((*ppstSubTree).pstLeft).cast());
        }
        VosAvlRotateRight(ppstSubTree.cast());
    }
    return;
}
```

Source:
```c
static void RapidlzCopyMatchFast(uint8_t *dst, uint8_t *match, uint16_t offset, uint32_t length)
{
    uint8_t *dstCurr = dst;
    uint8_t *matchPtr = match;
    if (offset >= RAPIDLZ_SIXTEEN_BYTE)
    {
        RapidlzCopyLiteralsFast(matchPtr, dstCurr, length);
        return;
    }
    for (int i = 0; i < RAPIDLZ_EIGHT_BYTE; ++i)
    {
        dstCurr[i] = matchPtr[i];
    }
    if (length <= RAPIDLZ_EIGHT_BYTE)
    {
        return;
    }
    uint8_t *dstEnd = dstCurr + length;
    if (offset < RAPIDLZ_EIGHT_BYTE)
    {
        matchPtr += g_overlapOffAddVal[offset];
        dstCurr += RAPIDLZ_EIGHT_BYTE;
    }
    do
    {
        RapidlzCopy8Byte(dstCurr, matchPtr);
        dstCurr += RAPIDLZ_EIGHT_BYTE;
        matchPtr += RAPIDLZ_EIGHT_BYTE;
    } while (dstCurr < dstEnd);
}
```

Translation:
```rust
pub fn RapidlzCopyMatchFast(mut dst: Ptr<u8>, mut r#match: Ptr<u8>, mut offset: u16, mut length: u32) {
    let mut dstCurr: Ptr<u8> = dst.cast();
    let mut matchPtr: Ptr<u8> = r#match.cast();
    if (offset >= RAPIDLZ_SIXTEEN_BYTE!()).as_bool() {
        RapidlzCopyLiteralsFast(matchPtr.cast(), dstCurr.cast(), length.cast());
        return;
    }
    c_for!(let mut i: i32 = 0; i < RAPIDLZ_EIGHT_BYTE!().cast(); i.prefix_plus_plus(); {
        dstCurr[i] = matchPtr[i].cast();
    });
    if (length <= RAPIDLZ_EIGHT_BYTE!()).as_bool() {
        return;
    }
    let mut dstEnd: Ptr<u8> = (dstCurr + length).cast();
    if (offset < RAPIDLZ_EIGHT_BYTE!()).as_bool() {
        matchPtr += (*g_overlapOffAddVal.lock())[offset];
        dstCurr += RAPIDLZ_EIGHT_BYTE!();
    }
    c_do!({
        RapidlzCopy8Byte(dstCurr.cast(), matchPtr.cast());
        dstCurr += RAPIDLZ_EIGHT_BYTE!();
        matchPtr += RAPIDLZ_EIGHT_BYTE!();
    } while dstCurr < dstEnd);
}
```

Source:
```c
static uint16_t RapidlzReadLE16Bit(const void *addr)
{
    if (RapidlzIsLE() != 0)
    {
        return *(const uint16_t *)addr;
    }
    uint8_t tmp1 = ((const uint8_t *)addr)[0];
    uint8_t tmp2 = ((const uint8_t *)addr)[1];
    return (uint16_t)(tmp1 + (tmp2 << 8));
}
```

Translation:
```rust
pub fn RapidlzReadLE16Bit(mut addr: Ptr<Void>) -> u16 {
    if (RapidlzIsLE() != 0).as_bool() {
        return (*addr.cast::<Ptr<u16>>()).cast();
    }
    let mut tmp1: u8 = ((addr.cast::<Ptr<u8>>())[0]).cast();
    let mut tmp2: u8 = ((addr.cast::<Ptr<u8>>())[1]).cast();
    return (tmp1 + (tmp2 << 8)).cast::<u16>();
}
```

Source:
```c
void *VOS_AVL_Find(AVL_TREE *pstTree, const void *pKey)
{
    AVL_NODE *pstNode;
    int iResult;
    if (pstTree == AVL_NULL_PTR)
    {
        return AVL_NULL_PTR;
    }
    pstNode = pstTree->pstRoot;
    while (pstNode != AVL_NULL_PTR)
    {
        iResult = pstTree->pfnCompare(pKey, pstNode->pKey);
        if (iResult > 0)
        {
            pstNode = pstNode->pstRight;
        }
        else if (iResult < 0)
        {
            pstNode = pstNode->pstLeft;
        }
        else
        {
            break;
        }
    }
    return ((pstNode != AVL_NULL_PTR) ? pstNode->pSelf : AVL_NULL_PTR);
}
```

Translation:
```rust
pub fn VOS_AVL_Find(mut pstTree: Ptr<AVL_TREE>, mut pKey: Ptr<Void>) -> Ptr<Void> {
    let mut pstNode: Ptr<AVL_NODE> = Default::default();
    let mut iResult: i32 = Default::default();
    if (pstTree == AVL_NULL_PTR!()).as_bool() {
        return AVL_NULL_PTR!();
    }
    pstNode = pstTree.pstRoot.cast();
    while (pstNode != AVL_NULL_PTR!()).as_bool() {
        iResult = (pstTree.pfnCompare)(pKey.cast(), pstNode.pKey.cast()).cast();
        if iResult > 0 {
            pstNode = pstNode.pstRight.cast();
        } else if iResult < 0 {
            pstNode = pstNode.pstLeft.cast();
        } else {
            break;
        }
    }
    return if pstNode != AVL_NULL_PTR!() { pstNode.pSelf.cast() } else { AVL_NULL_PTR!() };
}
```

Source:
```c
uint32_t BzpReadUInt24(InDeComdata *inData)
{
    uint8_t ch;
    uint32_t val = 0;
    ch = BzpReadBits(BZP_BITS8, inData);
    val = (val << BZP_BITS8) | ((uint32_t)ch);
    ch = BzpReadBits(BZP_BITS8, inData);
    val = (val << BZP_BITS8) | ((uint32_t)ch);
    ch = BzpReadBits(BZP_BITS8, inData);
    val = (val << BZP_BITS8) | ((uint32_t)ch);
    return val;
}
```

Translation:
```rust
pub fn BzpReadUInt24(mut inData: Ptr<InDeComdata>) -> u32 {
    let mut ch: u8 = Default::default();
    let mut val: u32 = 0;
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    val = (val << BZP_BITS8!()) | ch.cast::<u32>();
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    val = (val << BZP_BITS8!()) | ch.cast::<u32>();
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    val = (val << BZP_BITS8!()) | ch.cast::<u32>();
    return val.cast();
}
```

Source:
```c
int32_t BzpGetDictionaryList(InDeComdata *inData)
{
    int32_t ninUse = 0;
    bool use16[16] = {0};
    bool inUse[BZP_ASCII_SIZE] = {0};
    for (int32_t i = 0; i < BZP_GROUPS_ASCII; i++)
    {
        use16[i] = BzpReadBits(BZP_BIT, inData);
    }
    for (int32_t i = 0; i < BZP_GROUPS_ASCII; i++)
    {
        if (use16[i])
        {
            for (int32_t j = 0; j < BZP_CHARS_PER_GROUP_ASCII; j++)
            {
                inUse[i * BZP_GROUPS_ASCII + j] = BzpReadBits(BZP_BIT, inData);
            }
        }
    }
    for (int32_t i = 0; i < BZP_ASCII_SIZE; i++)
    {
        if (inUse[i])
        {
            inData->list[ninUse++] = i;
        }
    }
    return ninUse;
}
```

Translation:
```rust
pub fn BzpGetDictionaryList(mut inData: Ptr<InDeComdata>) -> i32 {
    let mut ninUse: i32 = 0;
    let mut use16: Array<bool, 16> = arr![false; 16];
    let mut inUse: Array<bool, { BZP_ASCII_SIZE!() }> = arr![false; BZP_ASCII_SIZE!()];
    c_for!(let mut i: i32 = 0; i < BZP_GROUPS_ASCII!().cast(); i.suffix_plus_plus(); {
        use16[i] = BzpReadBits(BZP_BIT!(), inData.cast()).cast();
    });
    c_for!(let mut i: i32 = 0; i < BZP_GROUPS_ASCII!().cast(); i.suffix_plus_plus(); {
        if use16[i].as_bool() {
            c_for!(let mut j: i32 = 0; j < BZP_CHARS_PER_GROUP_ASCII!(); j.suffix_plus_plus(); {
                inUse[i * BZP_GROUPS_ASCII!() + j] = BzpReadBits(BZP_BIT!(), inData.cast()).cast();
            });
        }
    });
    c_for!(let mut i: i32 = 0; i < BZP_ASCII_SIZE!().cast(); i.suffix_plus_plus(); {
        if inUse[i].as_bool() {
            inData.list[ninUse.suffix_plus_plus()] = i.cast();
        }
    });
    return ninUse.cast();
}
```

Source:
```c
void CmptlzLogWrite(size_t errorCode, const char *funcName, unsigned short line, const char *fmt, ...)
{
    va_list alist;
    char output[LOG_BUF_SIZE];
    int ret;
    size_t len;
    CmptlzLogFunc func = g_cmptlzLogFunc;
    if (func == NULL)
    {
        return;
    }
    ret = snprintf_s(output, LOG_BUF_SIZE, LOG_BUF_SIZE - 1, "\\n[Cmptlz-Log] Func=%s, Line=%u, Error=0x%zx\\n", funcName,
                     line, errorCode);
    if (ret < 0)
    {
        return;
    }
    len = (size_t)ret;
    va_start(alist, fmt);
    ret = vsnprintf_s(output + len, LOG_BUF_SIZE - len, LOG_BUF_SIZE - len - 1, fmt, alist);
    va_end(alist);
    if (ret < 0)
    {
        return;
    }
    func(output, strlen(output) + 1);
}
```

Translation:
```rust
pub fn CmptlzLogWrite(mut errorCode: usize, mut funcName: Ptr<u8>, mut line: u16, mut fmt: Ptr<u8>, mut alist: VaList) {
    // alist already initialized at parameter list
    let mut output: Array<u8, { LOG_BUF_SIZE!() }> = Default::default();
    let mut ret: i32 = Default::default();
    let mut len: usize = Default::default();
    let mut func: CmptlzLogFunc = *g_cmptlzLogFunc.lock();
    if (func == NULL!()).as_bool() {
        return;
    }
    ret = c_snprintf_s!(output, LOG_BUF_SIZE!(), LOG_BUF_SIZE!() - 1, cstr!("\\n[Cmptlz-Log] Func={}, Line={}, Error={}\\n"), funcName, line, errorCode);
    if (ret < 0).as_bool() {
        return;
    }
    len = ret.cast();
    // va_start not needed
    ret = c_vsnprintf_s!(output.cast::<Ptr<u8>>() + len, LOG_BUF_SIZE!() - len, LOG_BUF_SIZE!() - len - 1, fmt, alist);
    // va_end not needed
    if ret < 0 {
        return;
    }
    func(output.cast(), c_strlen!(output) + 1);
}
```

Source:
```c
int CmptLzDecSinglePacket(CmptLzDecCtx *decCtx, size_t dicPosLimit, const unsigned char *pSrcIn, size_t srcInLen,
                          size_t *psrcCostLen)
{
    int res;
    size_t lookAheadLen = 0;
    uint32_t newTempBufSize = decCtx->tempBufSize;
    unsigned char *oldTmpBuf = &(decCtx->tempBuf[decCtx->tempBufSize]);
    while (newTempBufSize < CMPTLZ_REQUIRED_INPUT_MAX && lookAheadLen < srcInLen)
    {
        decCtx->tempBuf[newTempBufSize++] = pSrcIn[lookAheadLen++];
    }
    const unsigned char *bufLimit = decCtx->tempBuf + newTempBufSize;
    res = CmptLzTryDecOnePacket(decCtx, &(decCtx->tempBuf[0]), &bufLimit);
    if (res == CMPTLZ_DEC_INPUT_EOF)
    {
        *psrcCostLen = lookAheadLen;
        decCtx->tempBufSize = newTempBufSize;
        return CMPTLZ_DEC_INPUT_EOF;
    }
    if (res == CMPT_ERROR_DATA)
    {
        return res;
    }
    decCtx->buf = &(decCtx->tempBuf[0]);
    res = CmptLzDecDirectProcess(decCtx, dicPosLimit, bufLimit);
    if ((res != CMPT_OK) || (bufLimit != decCtx->buf) || (bufLimit <= oldTmpBuf))
    {
        *psrcCostLen = 0;
        return CMPT_ERROR_DATA;
    }
    *psrcCostLen = (size_t)(bufLimit - oldTmpBuf);
    decCtx->tempBufSize = 0;
    return res;
}
```

Translation:
```rust
pub fn CmptLzDecSinglePacket(mut decCtx: Ptr<CmptLzDecCtx>, mut dicPosLimit: usize, mut pSrcIn: Ptr<u8>, mut srcInLen: usize, mut psrcCostLen: Ptr<usize>) -> i32 {
    let mut res: i32;
    let mut lookAheadLen: usize = 0;
    let mut newTempBufSize: u32 = decCtx.tempBufSize.cast();
    let mut oldTmpBuf: Ptr<u8> = (c_ref!(decCtx.tempBuf[0]) + decCtx.tempBufSize).cast();
    while (newTempBufSize < CMPTLZ_REQUIRED_INPUT_MAX!()).as_bool() && (lookAheadLen < srcInLen).as_bool() {
        decCtx.tempBuf[newTempBufSize] = pSrcIn[lookAheadLen].cast();
        newTempBufSize += 1;
        lookAheadLen += 1;
    }
    let mut bufLimit: Ptr<u8> = decCtx.tempBuf.cast::<Ptr<u8>>() + newTempBufSize;
    res = CmptLzTryDecOnePacket(decCtx.cast(), decCtx.tempBuf.cast(), c_ref!(bufLimit).cast()).cast();
    if (res == CMPTLZ_DEC_INPUT_EOF!()).as_bool() {
        *psrcCostLen = lookAheadLen.cast();
        decCtx.tempBufSize = newTempBufSize.cast();
        return CMPTLZ_DEC_INPUT_EOF!();
    }
    if (res == CMPT_ERROR_DATA!()).as_bool() {
        return res;
    }
    decCtx.buf = c_ref!(decCtx.tempBuf[0]).cast();
    res = CmptLzDecDirectProcess(decCtx.cast(), dicPosLimit.cast(), bufLimit.cast()).cast();
    if (res != CMPT_OK!()).as_bool() || (bufLimit != decCtx.buf).as_bool() || (bufLimit <= oldTmpBuf).as_bool() {
        *psrcCostLen = 0;
        return CMPT_ERROR_DATA!();
    }
    *psrcCostLen = (bufLimit - oldTmpBuf).cast::<usize>();
    decCtx.tempBufSize = 0;
    return res.cast();
}
```

Source:
```c
BzpHuffmanDecode *BzpHuffmanDecodeInit(int32_t blockSize)
{
    if (BZP_INVALID_BLOCK_SIZE(blockSize))
    {
        return NULL;
    }
    BzpHuffmanDecode *huffman = (BzpHuffmanDecode *)malloc(sizeof(BzpHuffmanDecode));
    if (huffman == NULL)
    {
        return NULL;
    }
    int32_t spaceSize = BZP_BASE_BLOCK_SIZE * blockSize / BZP_ELEMS_NUM_IN_ONE_GROUP;
    huffman->select = (int32_t *)malloc(spaceSize * sizeof(int32_t));
    if (huffman->select == NULL)
    {
        BzpHuffmanDecodeFinish(huffman);
    }
    (void)memset_s(huffman->base, sizeof(huffman->base), 0, sizeof(huffman->base));
    (void)memset_s(huffman->perm, sizeof(huffman->perm), 0, sizeof(huffman->perm));
    (void)memset_s(huffman->limit, sizeof(huffman->limit), 0, sizeof(huffman->limit));
    huffman->selectCnt = 0;
    huffman->deCodeNum = 0;
    return huffman;
}
```

Translation:
```rust
pub fn BzpHuffmanDecodeInit(mut blockSize: i32) -> Ptr<BzpHuffmanDecode> {
    if BZP_INVALID_BLOCK_SIZE!(blockSize).as_bool() {
        return NULL!();
    }
    let mut huffman: Ptr<BzpHuffmanDecode> = c_malloc!(c_sizeof!(BzpHuffmanDecode));
    if (huffman == NULL!()).as_bool() {
        return NULL!();
    }
    let mut spaceSize: i32 = BZP_BASE_BLOCK_SIZE!() * blockSize / BZP_ELEMS_NUM_IN_ONE_GROUP!();
    huffman.select = c_malloc!(spaceSize * c_sizeof!(i32));
    if (huffman.select == NULL!()).as_bool() {
        BzpHuffmanDecodeFinish(huffman.cast());
    }
    c_memset_s!(huffman.base, c_sizeofval!(huffman.base), 0, c_sizeofval!(huffman.base)).cast::<Void>();
    c_memset_s!(huffman.perm, c_sizeofval!(huffman.perm), 0, c_sizeofval!(huffman.perm)).cast::<Void>();
    c_memset_s!(huffman.limit, c_sizeofval!(huffman.limit), 0, c_sizeofval!(huffman.limit)).cast::<Void>();
    huffman.selectCnt = 0;
    huffman.deCodeNum = 0;
    return huffman.cast();
}
```
"""

macro_function_prompt = """\
Translate the C Code to Rust. 
You need to translate the macro only.
Notice that: 
Remember when translating macros, add `pub(crate)` to the macro definition to make it visible.
When using another macros in C, you should use macros in Rust with the same name. For example, `#define MY_MACRO ANOTHER_MACRO(ANOTHER_MACRO2)` should be translated to `macro_rules! MY_MACRO { () => { ANOTHER_MACRO!(ANOTHER_MACRO2!()) } }\n pub(crate) use MY_MACRO;`. Notice that C original macros, like __FILE__ and __LINE__, should also be translated to __FILE__!() and __LINE__!() in Rust.
If macros are used as comparision like 'if (MY_MACRO < a)', you should ensure the macro is placed at right side. For example, `if (MY_MACRO < a)` should be translated to `if a > MY_MACRO!()`.
When using the C string literals, use cstr! macro to translate it to Rust. For example, `#define MY_STR "Hello, World!"` should be translated to `macro_rules! MY_STR { () => { cstr!("Hello, World!") } }\n pub(crate) use MY_STR;`.
When find a C reference operator `&`, use c_ref! macro to translate it to Rust. For example, `int a = &b;` should be translated to `let mut a = c_ref!(b);`. For sizeof operator, if it is used with a type, translate it with c_sizeof! macro, for example, `sizeof(int)` should be translated to `c_sizeof!(int)`. If it is used with a variable, translate it with c_sizeofval! macro, for example, `sizeof(a)` should be translated to `c_sizeofval!(a)`.
Pointers in C should be translated to Ptr<T> in Rust, and void* should be translated to VoidPtr. char and unsigned char should all be translated to u8.
When there are type casting of pointers, use `.cast<Ptr<T>>()` method in Rust, do not use raw pointers like *const T. For example, `int *a = (int *)b;` should be translated to `let mut a = b.cast::<Ptr<i32>>();`, and `b = *(int16_t*)((uint32_t*)a + 8)` should be translated to `let mut b = *(a.cast::<Ptr<u32>>() + 8).cast::<Ptr<i16>>();`. 
Notice that always use `+` and `-` operator like original C, DO NOT use any Rust pointer method like `add()` or `offset()`. The `++` and `--` operators are not available in Rust, use .plus_plus(), .minus_minus() for suffix increment and decrement, and plus_plus!() and minus_minus!() for prefix increment and decrement. For example, `a++` -> `a.plus_plus()`, `++a` -> `plus_plus!(a)`.
However, if the type casting is not pointers but numbers, use `as` keyword in Rust. For example, `(uint32_t)a` should be translated to `a as u32`.
When passing a variable to a function, if it is not explicitly casted, add a .cast() method to the variable WITHOUT ANY TYPES. For example, `MyFunction(a, b, c)` in C should be translated to `MyFunction(a.cast(), b.cast(), c.cast())` in Rust.
Notice that when indexing a struct member with another struct member, like `s.cmp[s.curr]`, it may violate Rust borrow checker. You should use a temporary variable to store the value of the first struct member, and translate it to Rust with the temporary variable. For example, `s.cmp[s.curr++] = b` should be translated to `let idx = s.curr.plus_plus(); s.cmp[idx] = b;`.

Source:
```c
#define INIT_MY_STRUCT(s, a_c)            \
    do                                    \
    {                                     \
        (s).a = a_c;                      \
        (s).b = (MyMemeber *)MY_NULL;     \
        (s).c = &a_c                      \
    } while (0)                           \
```

Translation:
```rust
macro_rules! INIT_MY_STRUCT { ($s:expr, $a_c:expr) => 
    {
        $s.a = $a_c;
        $s.b = MY_NULL!();
        $s.c = c_ref!($a_c);
    }
}
pub(crate) use INIT_MY_STRUCT;
```

Source:
```c
#define MY_SIZE_OF_MUL_8(t) (sizeof(t) * 8)
```

Translation:
```rust
macro_rules! MY_SIZE_OF_MUL_8 { ($t:ty) => { c_sizeof!($t) * 8 } }
pub(crate) use MY_SIZE_OF_MUL_8;
```


Source:
```c
#define MY_CONDITION(a, b) (MY_NULL != myCompareFunc(a, b))
```
Remenber to change the MY_NULL to the right side.

Translation:
```rust
macro_rules! MY_CONDITION { ($a:expr, $b:expr) => { myCompareFunc($a, $b) != MY_NULL!() } }
pub(crate) use MY_CONDITION;
```

Source:
```c
#define MY_COMPLEX_MACRO(a, b, c, d)           \
    do                                         \
    {                                          \
        MY_USED_MACRO_1(a, *b--);              \
        MY_USED_MACRO_2(c, &(++d));                \
        MY_USE_MACRO_WITH_PARAM(a, 0x7832683d, MY_LINENUM); \
        int i;
        for (i = 0; i < sizeof(c) / sizeof(c[0]); i++) \
        { \
            MY_Usedfunction1((uint8_t*)&c[i]); \
            myUsedFunction2((uint8)c[i]); \
            
        } \
    } while (0)
```

Translation:
```rust
macro_rules! MY_COMPLEX_MACRO { ($a:expr, $b:expr, $c:expr, $d:expr) => 
    {
        MY_USED_MACRO_1!($a, *$b.minus_minus());
        MY_USED_MACRO_2!($c, c_ref!(plus_plus!($d)));
        MY_USE_MACRO_WITH_PARAM!($a, 0x7832683d, MY_LINENUM!());
        let mut i: i32;
        c_for!(i = 0; i < (c_sizeofval!($c) / c_sizeofval!($c[0])).cast(); i.plus_plus(); {
            MY_Usedfunction1(c_ref!($c[i]).cast::<Ptr<u8>>());
            myUsedFunction2($c[i] as u8);
        });
    }
}
pub(crate) use MY_COMPLEX_MACRO;
```

Source:
```c
#define MY_STRUCT_MACRO(s)                     \
    do                                         \
    {                                          \
        int i;                                 \
        for (i = 0; i < sizeof(s.arr) / sizeof(s.arr[0]); i++) \
        { \
            s.cmp[s.curr] = (uint8_t)(s.arr[i]); \
            s.curr += 1;
            s.cmp[s.curr] = (uint8_t)(s.arr1[s.curr] >> 8); \
            s.curr += 1;
            s.cmp[s.curr] = (uint8_t)(s.arr1[s.curr] >> 16); \
            s.curr += 1;
            s.cmp[s.curr] = (uint8_t)(s.arr1[s.curr] >> 24); \
            s.curr += 1;
        } \
    } while (0)
```

Translation:
```rust
macro_rules! MY_STRUCT_MACRO { ($s:expr) =>
    {
        let mut i: i32;
        c_for!(i = 0; i < (c_sizeofval!($s.arr) / c_sizeofval!($s.arr[0])).cast(); i.suffix_plus_plus(); {
            let idx = $s.curr; // bypass the borrow checker
            $s.cmp[idx] = $s.arr[i] as u8;
            $s.curr += 1;
            let idx = $s.curr; // bypass the borrow checker
            $s.cmp[idx] = ($s.arr1[$s.curr] >> 8) as u8;
            $s.curr += 1;
            let idx = $s.curr; // bypass the borrow checker
            $s.cmp[idx] = ($s.arr1[$s.curr] >> 16) as u8;
            $s.curr += 1;
            let idx = $s.curr; // bypass the borrow checker
            $s.cmp[idx] = ($s.arr1[$s.curr] >> 24) as u8;
            $s.curr += 1;
        });
    }
}
pub(crate) use MY_STRUCT_MACRO;
```

Source:
```c
#define MY_WHILE_PTR_LAST_NOT_EQUAL(ptr1, ptr2) \
    do                                         \
    {                                          \
        while ((ptr1[-1]) != (ptr2[-1]))       \
        {                                      \
            ptr1--;                            \
            ptr2--;                            \
        }                                      \
    } while (0)
```

Translation:
```rust
macro_rules! MY_WHILE_PTR_LAST_NOT_EQUAL { ($ptr1:expr, $ptr2:expr) =>
    {
        while $ptr1[-1] != $ptr2[-1]
        {
            $ptr1.minus_minus();
            $ptr2.minus_minus();
        }
    }
}
pub(crate) use MY_WHILE_PTR_LAST_NOT_EQUAL;
```

Do not care about __builtin_expect() in C code, they are not necessary in Rust.  

Source:
```c
#define MY_BUILTIN_PREDICT(X) __builtin_expect(!!(X), 1)
```

Translation:
```rust
macro_rules! MY_BUILTIN_PREDICT { ($X:expr) => { X } }
pub(crate) use MY_BUILTIN_PREDICT;
```

When translate macro with va-args, you should use the following pattern:

Source:
```c
#define MY_VARGS_M(fmt, args...)                                                                           \
    do                                                                                                                 \
    {                                                                                                                  \
        MyVargsFunction(MYFILENAME, __LINE__, fmt, ##args);                                 \
    } while (0)
```

Translation:
```rust
macro_rules! MY_VARGS_M {
    ($fmt:expr) => {
        MyVargsFunction(MYFILENAME!().cast(), __LINE__!().cast(), $fmt.cast(), &[]);
    };
    ($fmt:expr, $($args:expr),*) => {
        MyVargsFunction(MYFILENAME!().cast(), __LINE__!().cast(), $fmt.cast(), &[$(&$args), *]);
    }
}
pub(crate) use MY_VARGS_M;
```
"""

macro_prompt = """\
Translate the C Code to Rust. 
You need to translate the macro only.
Notice that: 
Remember when translating macros, add `pub(crate)` to the macro definition to make it visible.
When using another macros in C, you should use macros in Rust with the same name. For example, `#define MY_MACRO ANOTHER_MACRO(ANOTHER_MACRO2)` should be translated to `macro_rules! MY_MACRO { () => { ANOTHER_MACRO!(ANOTHER_MACRO2!()) } }\n pub(crate) use MY_MACRO;`. Notice that C original macros, like __FILE__ and __LINE__, should also be translated to __FILE__!() and __LINE__!() in Rust.
Any expression as a condition of `if` or `while` or ternary operator should be append with `.as_bool()` method in Rust. For example, `(MyFunc(1)) ? 1 : 0` should be translated to `if MyFunc(1).as_bool() { 1 } else { 0 }`.
When using the C string literals, use cstr! macro to translate it to Rust. For example, `#define MY_STR "Hello, World!"` should be translated to `macro_rules! MY_STR { () => { cstr!("Hello, World!") } }\n pub(crate) use MY_STR;`.
When translating a macro that represents NULL pointer, use NULL!() macro in Rust. For example, `#define MY_NULL 0` should be translated to `macro_rules! MY_NULL { () => { NULL!() } }\n pub(crate) use MY_NULL;`. If it represents true or false, just use the numbers in C, for example, `#define MY_TRUE 1` should be translated to `macro_rules! MY_TRUE { () => { 1 } }\n pub(crate) use MY_TRUE;`, and `#define MY_FALSE 0` should be translated to `macro_rules! MY_FALSE { () => { 0 } }\n pub(crate) use MY_FALSE;`.
When using global variables that start with `g_`, use the lock() method to get reference of the variable. For example, `int a = g_a;` should be translated to `let mut a = (*g_a.lock());`, and `g_a = 10;` should be translated to `(*g_a.lock()) = 10;`.
When find a C reference operator `&`, use c_ref! macro to translate it to Rust. For example, `int a = &b;` should be translated to `let mut a = c_ref!(b);`.
If the original number literal in C has type, like `10U`, just ignore it and translate to Rust number without types directly, i.e. `10`. For example, `#define MY_NUM 10U` should be translated to `macro_rules! MY_NUM { () => { 10 } }\n pub(crate) use MY_NUM;`. However, if it is explicitly casted, like `(size_t)10`, you should translate it to Rust with the cast, with original `isize` literal, i.e. `(10isize as usize)`. 

Source:
```c
#define MY_NUMA 10U
```

Translation:
```rust
macro_rules! MY_NUMA { () => { 10 } }
pub(crate) use MY_NUMA;
```

Source:
```c
#define MY_NUMB 10LL
```

Translation:
```rust
macro_rules! MY_NUMB { () => { 10 } }
pub(crate) use MY_NUMB;
```

Source:
```c
#define MY_FFFF 0xffffL
```

Translation:
```rust
macro_rules! MY_FFFF { () => { 0xffff } }
pub(crate) use MY_FFFF;
```

Source:
```c
#define MYHEX 0x30
```

Translation:
```rust
macro_rules! MYHEX { () => { 0x30 } }
pub(crate) use MYHEX;
```

Source:
```c
#define MY_MALLOC_8 malloc(8)
```

Translation:
```rust
macro_rules! MY_MALLOC_8 { () => { malloc(8) } }
pub(crate) use MY_MALLOC_8;
```

Source:
```c
#define MY_NULL 0
```

Translation:
```rust
macro_rules! MY_NULL { () => { NULL!() } }
pub(crate) use MY_NULL;
```

Source:
```c
#define MY_EOK 0
```

Translation:
```rust
macro_rules! MY_EOK { () => { 0 } }
pub(crate) use MY_EOK;
```

Source:
```c
#define MY_NEG (uint8_t)~0
```

Translation:
```rust
macro_rules! MY_NEG { () => { (!0isize) as u8 } }
pub(crate) use MY_NEG;
```

Source:
```c
#define MY_SELECTION (MY_CONDITION!()) ? MY_SELECTION_TRUE : MY_SELECTION_FALSE
```

Translation:
```rust
macro_rules! MY_SELECTION { () => { if MY_CONDITION!().as_bool() > 10 { MY_SELECTION_TRUE!() } else { MY_SELECTION_FALSE!() } } }
pub(crate) use MY_SELECTION;
```

Source:
```c
#define MY_U8_MINUS (uint8_t)-1
```

Translation:
```rust
macro_rules! MY_U8_MINUS { () => { (-1isize) as u8 } }
```

Source:
```c
#define MY_CALL_FUNC_WITH_CHAR My_calledFunc('a')
```

Translation:
```rust
macro_rules! MY_CALL_FUNC_WITH_CHAR { () => { My_calledFunc(b'a' as u8) } }
```
"""

delim_repair_prompt = """\
Fix the compiler bugs in the following Rust code with provided compiler error messagesm, possibly because of mismatched parens.
Only fix lines that have unmatched parens bugs, don't modify any other code.

Source:
```rust
pub fn rb_tree_insert_case4(mut tree: Ptr<RBTree>, mut node: Ptr<RBTreeNode>) {
    let mut next_node: Ptr<RBTreeNode> = Default::default();
    let mut side: RBTreeNodeSide = Default::default();
    
    side = rb_tree_node_side(node);
    if (side != rb_tree_node_side(node.parent) {
        next_node = node.parent;
        rb_tree_rotate(tree, node.parent, (1 - side));
    } else {
        next_node = node;
    }
    rb_tree_insert_case5(tree, next_node);
}
```

Error Message:
"error: mismatched closing delimiter: `}`   --> src/src/rb_tree_c.rs:154:8\n    |\n148 | pub fn rb_tree_insert_case4(mut tree: Ptr<RBTree>, mut node: Ptr<RBTreeNode>) {\n    |                                                                               - closing delimiter possibly meant for this\n...\n154 |     if (side != rb_tree_node_side(node.parent).cast().as_bool() {\n    |        ^ unclosed delimiter\n...\n163 | }\n    | ^ mismatched closing delimiter\n\nerror: could not compile `my_proj` (lib) due to 1 previous error\n"

Fixed Code:
```rust
pub fn rb_tree_insert_case4(mut tree: Ptr<RBTree>, mut node: Ptr<RBTreeNode>) {
    let mut next_node: Ptr<RBTreeNode> = Default::default();
    let mut side: RBTreeNodeSide = Default::default();
    
    side = rb_tree_node_side(node);
    if (side != rb_tree_node_side(node.parent)) {
        next_node = node.parent;
        rb_tree_rotate(tree, node.parent, (1 - side));
    } else {
        next_node = node;
    }
    rb_tree_insert_case5(tree, next_node);
}
```

Source:
```rust
pub fn rb_tree_insert(mut tree: Ptr<RBTree>, mut key: RBTreeKey, mut value: RBTreeValue) -> Ptr<RBTreeNode> {
    let mut node: Ptr<RBTreeNode>;
    let mut rover: Ptr<Ptr<RBTreeNode>>;
    let mut parent: Ptr<RBTreeNode>;
    let mut side: RBTreeNodeSide;
    
    node = c_malloc!(c_sizeof!(RBTreeNode));
    
    if (node == NULL!()) {
        return NULL!();
    }
    node.key = key;
    node.value = value;
    node.color = RB_TREE_NODE_RED!();
    node.children[RB_TREE_NODE_LEFT!()] = NULL!();
    node.children[RB_TREE_NODE_RIGHT!()] = NULL!();
    
    parent = NULL!();
    rover = c_ref!(tree.root_node);
    
    while (*rover != NULL!()) {
        parent = *rover;
        if (tree.compare_func(value, (*rover).value) < 0 {
            side = RB_TREE_NODE_LEFT!();
        } else {
            side = RB_TREE_NODE_RIGHT!();
        }
        
        rover = c_ref!((*rover).children[side]);
    }
    
    *rover = node;
    node.parent = parent;
    rb_tree_insert_case1(tree, node);
    tree.num_nodes.prefix_plus_plus();
    return node;
}
```

Error Message:
"error: mismatched closing delimiter: `}`\n   --> src/src/rb_tree_c.rs:194:12\n    |\n191 |     while (*rover != NULL!()).as_bool() {\n    |                                         - closing delimiter possibly meant for this\n...\n194 |         if (tree.compare_func(value.cast(), (*rover).value.cast()) < 0 {\n    |            ^ unclosed delimiter\n...\n201 |     }\n    |     ^ mismatched closing delimiter\n\nerror: could not compile `my_proj` (lib) due to 1 previous error\n"

Fixed Code:
```rust
pub fn rb_tree_insert(mut tree: Ptr<RBTree>, mut key: RBTreeKey, mut value: RBTreeValue) -> Ptr<RBTreeNode> {
    let mut node: Ptr<RBTreeNode>;
    let mut rover: Ptr<Ptr<RBTreeNode>>;
    let mut parent: Ptr<RBTreeNode>;
    let mut side: RBTreeNodeSide;
    
    node = c_malloc!(c_sizeof!(RBTreeNode));
    
    if (node == NULL!()) {
        return NULL!();
    }
    node.key = key;
    node.value = value;
    node.color = RB_TREE_NODE_RED!();
    node.children[RB_TREE_NODE_LEFT!()] = NULL!();
    node.children[RB_TREE_NODE_RIGHT!()] = NULL!();
    
    parent = NULL!();
    rover = c_ref!(tree.root_node);
    
    while (*rover != NULL!()) {
        parent = *rover;
        if (tree.compare_func(value, (*rover).value) < 0) {
            side = RB_TREE_NODE_LEFT!();
        } else {
            side = RB_TREE_NODE_RIGHT!();
        }
        
        rover = c_ref!((*rover).children[side]);
    }
    
    *rover = node;
    node.parent = parent;
    rb_tree_insert_case1(tree, node);
    tree.num_nodes.prefix_plus_plus();
    return node;
}
```
"""


repair_prompt = """\
Fix the compiler bugs in the following Rust code with provided compiler error messagesm.
Fix these bugs according to the compiler informations:
1. Type mismatch: use `cast::<T>` to cast to original type to the targeted type.
2. Wrong function as struct field calling: `my_struct.my_func(a, b)` should be corrected as `(my_struct.my_func)(a, b)`.
3. Constant/Macro confusion: `a > MY_MACRO` should be `a > MY_MACRO!()`, and `b > my_constant!()` should be `b > my_constant`
4. Other bugs, just repair the corresponding line with the reference of error messages. 

Source:
```rust
pub fn binomial_tree_unref(mut tree: Ptr<BinomialTree>) {
    let mut i: i32 = Default::default();

    if (tree == NULL!()) {
        return;
    }
    
    tree.refcount.suffix_minus_minus()
    
    if (tree.refcount == 0) {
        c_for!(i = 0; i < tree.order; i.prefix_plus_plus(); {
            binomial_tree_unref(tree.subtrees[i]);
        });
        c_free!(tree.subtrees);
        c_free!(tree);
    }
}
```

Error Message:
"error[E0308]: mismatched types\n  --> src/src/binomial_heap_c.rs:55:27\n   |\n55 |         c_for!(i = 0; i < tree.order; i.prefix_plus_plus(); {\n   |                       -   ^^^^^^^^^^ expected `i32`, found `u16`\n   |                       |\n   |                       expected because this is `i32`\n   |\nhelp: you can convert a `u16` to an `i32`\n   |\n55 |         c_for!(i = 0; i < tree.order.into(); i.prefix_plus_plus(); {\n   |                                     +++++++\n\nFor more information about this error, try `rustc --explain E0308`.\nerror: could not compile `my_proj` (lib) due to 1 previous error\n"

Fixed Code:
```rust
pub fn binomial_tree_unref(mut tree: Ptr<BinomialTree>) {
    let mut i: i32 = Default::default();

    if (tree == NULL!()) {
        return;
    }
    
    tree.refcount.suffix_minus_minus()
    
    if (tree.refcount == 0) {
        c_for!(i = 0; i < tree.order.cast::<i32>(); i.prefix_plus_plus(); {
            binomial_tree_unref(tree.subtrees[i]);
        });
        c_free!(tree.subtrees);
        c_free!(tree);
    }
}
```

Source:
```rust
pub fn rb_tree_insert(mut tree: Ptr<RBTree>, mut key: RBTreeKey, mut value: RBTreeValue) -> Ptr<RBTreeNode> {
    let mut node: Ptr<RBTreeNode>;
    let mut rover: Ptr<Ptr<RBTreeNode>>;
    let mut parent: Ptr<RBTreeNode>;
    let mut side: RBTreeNodeSide;

    node = c_malloc!(c_sizeof!(RBTreeNode));

    if (node == NULL!()) {
        return NULL!();
    }

    node.key = key;
    node.value = value;
    node.color = RB_TREE_NODE_RED!();
    node.children[RB_TREE_NODE_LEFT!()] = NULL!();
    node.children[RB_TREE_NODE_RIGHT!()] = NULL!();

    parent = NULL!();
    rover = c_ref!(tree.root_node);

    while (*rover != NULL!()) {
        parent = *rover;

        if (tree.compare_func(value, (*rover).value) < 0) {
            side = RB_TREE_NODE_LEFT!();
        } else {
            side = RB_TREE_NODE_RIGHT!();
        }

        rover = c_ref!((*rover).children[side]);
    }

    *rover = node;
    node.parent = parent;

    rb_tree_insert_case1(tree, node);

    tree.num_nodes.prefix_plus_plus();

    return node;
}
```

Error Message:
"error: mismatched closing delimiter: `}`\n   --> src/src/rb_tree_c.rs:194:12\n    |\n191 |     while (*rover != NULL!()).as_bool() {\n    |                                         - closing delimiter possibly meant for this\n...\n194 |         if (tree.compare_func(value.cast(), (*rover).value.cast()) < 0 {\n    |            ^ unclosed delimiter\n...\n201 |     }\n    |     ^ mismatched closing delimiter\n\nerror: could not compile `my_proj` (lib) due to 1 previous error\n"

Fixed Code:
```rust
pub fn rb_tree_insert(mut tree: Ptr<RBTree>, mut key: RBTreeKey, mut value: RBTreeValue) -> Ptr<RBTreeNode> {
    let mut node: Ptr<RBTreeNode>;
    let mut rover: Ptr<Ptr<RBTreeNode>>;
    let mut parent: Ptr<RBTreeNode>;
    let mut side: RBTreeNodeSide;

    node = c_malloc!(c_sizeof!(RBTreeNode));

    if (node == NULL!()) {
        return NULL!();
    }

    node.key = key;
    node.value = value;
    node.color = RB_TREE_NODE_RED!();
    node.children[RB_TREE_NODE_LEFT!()] = NULL!();
    node.children[RB_TREE_NODE_RIGHT!()] = NULL!();

    parent = NULL!();
    rover = c_ref!(tree.root_node);

    while (*rover != NULL!()) {
        parent = *rover;

        if ((tree.compare_func)(value, (*rover).value) < 0) {
            side = RB_TREE_NODE_LEFT!();
        } else {
            side = RB_TREE_NODE_RIGHT!();
        }

        rover = c_ref!((*rover).children[side]);
    }

    *rover = node;
    node.parent = parent;

    rb_tree_insert_case1(tree, node);

    tree.num_nodes.prefix_plus_plus();

    return node;
}
```

Source:
```rust
pub fn string_hash(mut string: Ptr<Void>) -> u32 {
    let mut result: u32 = 5381;
    let mut p: Ptr<u8> = string.cast::<Ptr<u8>>();

    while (*p != '\\0') {
        result = (result << 5) + result + (*p).cast::<u32>();
        p += 1;
    }

    return result;
}
```

Error Message:
"error[E0308]: mismatched types\n --> src/src/hash_string_c.rs:8:18\n  |\n8 |     while (*p != '\\0') {\n  |            --    ^^^^ expected `u8`, found `char`\n  |            |\n  |            expected because this is `u8`\n  |\nhelp: if you meant to write a byte literal, prefix with `b`\n  |\n8 |     while (*p != b'\\0') {\n  |                  ~~~~~\n\nFor more information about this error, try `rustc --explain E0308`.\nerror: could not compile `my_proj` (lib) due to 1 previous error\n"

Fixed Code:
```rust
pub fn string_hash(mut string: Ptr<Void>) -> u32 {
    let mut result: u32 = 5381;
    let mut p: Ptr<u8> = string.cast::<Ptr<u8>>();

    while (*p != b'\\0' as u8) {
        result = (result << 5) + result + (*p).cast::<u32>();
        p += 1;
    }

    return result;
}
```

Source:
```rust
pub fn trie_free_list_pop(mut list: Ptr<Ptr<TrieNode>>) -> Ptr<TrieNode> {
    let mut result: Ptr<TrieNode>;

    result = *list;
    *list = result.data;

    return result;
}
```

Error Message:
"error[E0308]: mismatched types\n  --> src/src/trie_c.rs:46:13\n   |\n46 |     *list = result.data;\n   |     -----   ^^^^^^^^^^^ expected `Ptr<_TrieNode>`, found `Ptr<u8>`\n   |     |\n   |     expected due to the type of this binding\n   |\n   = note: expected struct `memory::ptr::Ptr<_TrieNode>`\n              found struct `memory::ptr::Ptr<u8>`\nhelp: consider removing the tuple struct field `data`\n   |\n46 -     *list = result.data;\n46 +     *list = result;\n   |\n\nFor more information about this error, try `rustc --explain E0308`.\nerror: could not compile `my_proj` (lib) due to 1 previous error\n"

Fixed Code:
```rust
pub fn trie_free_list_pop(mut list: Ptr<Ptr<TrieNode>>) -> Ptr<TrieNode> {
    let mut result: Ptr<TrieNode>;

    result = *list;
    *list = result.data.cast::<Ptr<TrieNode>>();

    return result;
}
```

Source:
```rust
pub fn set_allocate_table(mut set: Ptr<Set>) -> i32 {
    if (set.prime_index < set_num_primes!()) {
        let tmp0 = set.prime_index;
        set.table_size = set_primes[tmp0];
    } else {
        set.table_size = (set.entries * 10);
    }
    set.table = c_calloc!(set.table_size, c_sizeof!(Ptr<SetEntry>));
    return (set.table != NULL!()).cast::<i32>();
}
```

Error Message:
"error: cannot find macro `set_num_primes` in this scope\n  --> src/src/set_c.rs:35:27\n   |\n35 |     if (set.prime_index < set_num_primes!()) {\n   |                           ^^^^^^^^^^^^^^\n   |\n   = note: `set_num_primes` is in scope, but it is a constant, not a macro\n\nerror: could not compile `my_proj` (lib) due to 1 previous error\n"

Fixed Code:
```rust
pub fn set_allocate_table(mut set: Ptr<Set>) -> i32 {
    if (set.prime_index < set_num_primes) {
        let tmp0 = set.prime_index;
        set.table_size = set_primes[tmp0];
    } else {
        set.table_size = (set.entries * 10);
    }
    set.table = c_calloc!(set.table_size, c_sizeof!(Ptr<SetEntry>));
    return (set.table != NULL!()).cast::<i32>();
}
```
"""