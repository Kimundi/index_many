Disclaimer: It's been a while since I contributed to the main Rust repo, apologies in advance if this is large enough already that it should've been an RFC.

---

# Update:

- Based on feedback, removed the `&[T]` variant of this API, and removed the requirements for the indices to be sorted.

# Description

This adds the following slice methods to `core`:

```rust
impl<T> [T] {
    pub unsafe fn get_many_unchecked_mut<const N: usize>(&mut self, indices: [usize; N]) -> [&mut T; N];
    pub fn get_many_mut<const N: usize>(&mut self, indices: [usize; N]) -> Option<[&mut T; N]>;
}
```

This allows creating multiple mutable references to disjunct positions in a slice, which previously required writing some awkward code with `split_at_mut()` or `iter_mut()`. For the bound-checked variant, the indices are checked against each other and against the bounds of the slice, which requires `N * (N + 1) / 2` comparison operations.

This has a proof-of-concept standalone implementation here: https://crates.io/crates/index_many

Care has been taken that the implementation passes miri borrow checks, and generates straight-forward assembly (though this was only checked on x86_64).

# Example

```rust
let v = &mut [1, 2, 3, 4];
let [a, b] = v.get_many_mut([0, 2]).unwrap();
std::mem::swap(a, b);
*v += 100;
assert_eq!(v, &[3, 2, 101, 4]);
```

# Codegen Examples

<details>
  <summary>Click to expand!</summary>

Disclaimer: Taken from local tests with the standalone implementation.

## Unchecked Indexing:

```rust
pub unsafe fn example_unchecked(slice: &mut [usize], indices: [usize; 3]) -> [&mut usize; 3] {
    slice.get_many_unchecked_mut(indices)
}
```

```nasm
example_unchecked:
 mov     rcx, qword, ptr, [r9]
 mov     r8, qword, ptr, [r9, +, 8]
 mov     r9, qword, ptr, [r9, +, 16]
 lea     rcx, [rdx, +, 8*rcx]
 lea     r8, [rdx, +, 8*r8]
 lea     rdx, [rdx, +, 8*r9]
 mov     qword, ptr, [rax], rcx
 mov     qword, ptr, [rax, +, 8], r8
 mov     qword, ptr, [rax, +, 16], rdx
 ret
```

## Checked Indexing (Option):

```rust
pub unsafe fn example_option(slice: &mut [usize], indices: [usize; 3]) -> Option<[&mut usize; 3]> {
    slice.get_many_mut(indices)
}
```

```nasm
 mov     r10, qword, ptr, [r9, +, 8]
 mov     rcx, qword, ptr, [r9, +, 16]
 cmp     rcx, r10
 je      .LBB0_7
 mov     r9, qword, ptr, [r9]
 cmp     rcx, r9
 je      .LBB0_7
 cmp     rcx, r8
 jae     .LBB0_7
 cmp     r10, r9
 je      .LBB0_7
 cmp     r9, r8
 jae     .LBB0_7
 cmp     r10, r8
 jae     .LBB0_7
 lea     r8, [rdx, +, 8*r9]
 lea     r9, [rdx, +, 8*r10]
 lea     rcx, [rdx, +, 8*rcx]
 mov     qword, ptr, [rax], r8
 mov     qword, ptr, [rax, +, 8], r9
 mov     qword, ptr, [rax, +, 16], rcx
 ret
.LBB0_7:
 mov     qword, ptr, [rax], 0
 ret
```

## Checked Indexing (Panic):

```rust
pub fn example_panic(slice: &mut [usize], indices: [usize; 3]) -> [&mut usize; 3] {
    let len = slice.len();
    match slice.get_many_mut(indices) {
        Some(s) => s,
        None => {
            let tmp = indices;
            index_many::sorted_bound_check_failed(&tmp, len)
        }
    }
}
```

```nasm
example_panic:
 sub     rsp, 56
 mov     rax, qword, ptr, [r9]
 mov     r10, qword, ptr, [r9, +, 8]
 mov     r9, qword, ptr, [r9, +, 16]
 cmp     r9, r10
 je      .LBB0_6
 cmp     r9, rax
 je      .LBB0_6
 cmp     r9, r8
 jae     .LBB0_6
 cmp     r10, rax
 je      .LBB0_6
 cmp     rax, r8
 jae     .LBB0_6
 cmp     r10, r8
 jae     .LBB0_6
 lea     rax, [rdx, +, 8*rax]
 lea     r8, [rdx, +, 8*r10]
 lea     rdx, [rdx, +, 8*r9]
 mov     qword, ptr, [rcx], rax
 mov     qword, ptr, [rcx, +, 8], r8
 mov     qword, ptr, [rcx, +, 16], rdx
 mov     rax, rcx
 add     rsp, 56
 ret
.LBB0_6:
 mov     qword, ptr, [rsp, +, 32], rax
 mov     qword, ptr, [rsp, +, 40], r10
 mov     qword, ptr, [rsp, +, 48], r9
 lea     rcx, [rsp, +, 32]
 mov     edx, 3
 call    index_many::bound_check_failed
 ud2
```
</details>

# Extensions

There are multiple optional extensions to this.

## Indexing With Ranges

This could easily be expanded to allow indexing with `[I; N]` where `I: SliceIndex<Self>`.  I wanted to keep the initial implementation simple, so I didn't include it yet.

## Panicking Variant

We could also add this method:

```rust
impl<T> [T] {
    fn index_many_mut<const N: usize>(&mut self, indices: [usize; N]) -> [&mut T; N];
}
```

This would work similar to the regular index operator and panic with out-of-bound indices. The advantage would be that we could more easily ensure good codegen with a useful panic message, which is non-trivial with the `Option` variant.

This is implemented in the standalone implementation, and used as basis for the codegen examples here and there.
