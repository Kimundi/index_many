Proof of concept functions for (mutably) accessing a slice at multiple positions at once via an array of indices.

# Provided APIs

This crate implements multiple APIs:

- "simple": accepts a sorted array `[usize; N]` of indices.
- "simple_result": accepts a sorted array `[usize; N]` of indices, but with an `Result` based API.
- "slice_index": accepts a sorted array `[I; N]` of indices, where `I: SliceIndex<[T]>`.
- "generic": uses a generic `I: Indices<N>` that allows for more flexibility.

# Example

```rust
use index_many::generic::index_many_mut;

let mut v = vec![1, 2, 3, 4, 5];
let [a, b, c] = index_many_mut(&mut v, [0, 2, 4]);
*a += 10;
*b += 100;
*c += 1000;
assert_eq!(v, vec![11, 2, 103, 4, 1005]);
```

# Generated Assembly

The docs contain example functions with their x86_64 assembly codegen. See the [`crate::_doc_assembly`] module.
