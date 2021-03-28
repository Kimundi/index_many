Proof of concept functions for (mutably) accessing a slice at multiple positions at once via an array of indices.

# Provided APIs

This crate implements two APIs:

- "simple": only accepts a sorted array `[usize; N]` of indices.
- "generic": uses a generic `I: Indices<N>` that allows for more flexibility.

# Example

```rust
use index_many::generic::SliceExt;

let mut v = vec![1, 2, 3, 4, 5];
let [a, b, c] = v.index_many_mut([0, 2, 4]);
*a += 10;
*b += 100;
*c += 1000;
assert_eq!(v, vec![11, 2, 103, 4, 1005]);
```

# Example codegen

```rust
pub fn example(slice: &mut [usize], indices: [usize; 3]) -> [&mut usize; 3] {
    index_many::generic::index_many_mut(slice, indices)
}
```

```nasm
example:
 sub     rsp, 56
 mov     r10, qword, ptr, [r9]
 mov     rax, qword, ptr, [r9, +, 8]
 mov     r9, qword, ptr, [r9, +, 16]
 cmp     r9, r8
 jae     .LBB0_3
 cmp     r10, rax
 jae     .LBB0_3
 cmp     rax, r9
 jae     .LBB0_3
 lea     r8, [rdx, +, 8*r10]
 lea     rax, [rdx, +, 8*rax]
 lea     rdx, [rdx, +, 8*r9]
 mov     qword, ptr, [rcx], r8
 mov     qword, ptr, [rcx, +, 8], rax
 mov     qword, ptr, [rcx, +, 16], rdx
 mov     rax, rcx
 add     rsp, 56
 ret
.LBB0_3:
 mov     qword, ptr, [rsp, +, 32], r10
 mov     qword, ptr, [rsp, +, 40], rax
 mov     qword, ptr, [rsp, +, 48], r9
 lea     rcx, [rsp, +, 32]
 mov     edx, 3
 call    index_many::sorted_bound_check_failed
 ud2
```