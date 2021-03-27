Proof of concept functions for (mutably) accessing a slice at multiple positions at once via an array of indices.

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
