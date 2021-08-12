Disclaimer: It's been a while since I contributed to the main Rust repo, apologies in advance if this is large enough already that it should've been an RFC.

---

# Description

This adds the following slice methods to `core`:

```rust
impl<T> [T] {
    pub unsafe fn get_many_unchecked<const N: usize>(&self, indices: [usize; N]) -> [&T; N];
    pub unsafe fn get_many_unchecked_mut<const N: usize>(&mut self, indices: [usize; N]) -> [&mut T; N];
    pub fn get_many<const N: usize>(&self, indices: [usize; N]) -> Option<[&T; N]>;
    pub fn get_many_mut<const N: usize>(&mut self, indices: [usize; N]) -> Option<[&mut T; N]>;
}
```

This allows creating multiple mutable references to disjunct positions in a slice, which previously required writing some awkward code with `split_at_mut()` or `iter_mut()`. For the bound-checked variant, the indices are required to be sorted, as this requires only `N` comparisons.

This has a proof-of-concept standalone implementation here: https://crates.io/crates/index_many

Care has been taken that the implementation passes miri borrow checks, and generates straight-forward assembly (though this was only checked on x86_64).

The proposed non-`mut` methods do not add any functionality that is not already possible with existing safe APIs. They are merely there for symmetry, as the slice APIs generally tend to offer both mutability variants. (For example `split_at` and `split_at_mut`)

# Example

```rust
let v = &mut [1, 2, 3, 4];
let [a, b] = v.get_many_mut([0, 2]).unwrap();
std::mem::swap(a, b);
assert_eq!(v, &[3, 2, 1, 4]);
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
example_option:
 mov     r10, qword, ptr, [r9, +, 16]
 cmp     r10, r8
 jae     .LBB2_3
 mov     r8, qword, ptr, [r9]
 mov     rcx, qword, ptr, [r9, +, 8]
 cmp     r8, rcx
 jae     .LBB2_3
 cmp     rcx, r10
 jae     .LBB2_3
 lea     r8, [rdx, +, 8*r8]
 lea     rcx, [rdx, +, 8*rcx]
 lea     rdx, [rdx, +, 8*r10]
 mov     qword, ptr, [rax], r8
 mov     qword, ptr, [rax, +, 8], rcx
 mov     qword, ptr, [rax, +, 16], rdx
 ret
.LBB2_3:
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
</details>

# Extensions

There are multiple optional extensions to this.

## Indexing With Ranges

This could easily be expanded to allow indexing with `[I; N]` where `I: SliceIndex<Self>`.  I wanted to keep the initial implementation simple, so I didn't include it yet.

## Panicking Variant

We could also add these methods:

```rust
impl<T> [T] {
    fn index_many<const N: usize>(&self, indices: [usize; N]) -> [&T; N];
    fn index_many_mut<const N: usize>(&mut self, indices: [usize; N]) -> [&mut T; N];
}
```

This would work similar to the regular index operator and panic with out-of-bound indices. The advantage would be that we could more easily ensure good codegen with a useful panic message, which is non-trivial with the `Option` variant.

This is implemented in the standalone implementation, and used as basis for the codegen examples here and there.

## Sorted Requirement

Instead of requiring the indices array to be sorted, we could also:

- Require a custom `Presorted<N>` type that ensures the indices are sorted at construction, which allows the actual `get_many` call to only require a single bound check
- Allow arbitrary ordered indices, in which case `get_many` would require `O(N^2)` comparisons for simple codegen, or would need to sort the indices with `O(N log N)` operations to check them.

Both variants are implemented in the standalone implementation as well, but seem like poorer default choices.

## Weaker requirements on the `&[T]` API.

Right now the implementation for `&[T]` and `&mut [T]` is the same, even though `&[T]` can have weaker restrictions: Creating multiple references to the same index values would be fine, so the safe API would not need to require a sorted array of mutal disjunct indices, and could instead just require any indices that are in bounds.

However, lifting these restrictions would make the API inconsistent to the mutable one, and can also be done at any point in the future, so the current implementation is the conservative choice here.
