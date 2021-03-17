# index_many

Proof of concept for a method to (mutably) index a slice at multiple indices at once.

# Example

```rust
let mut v = vec![1, 2, 3, 4, 5];
let [a, b, c] = index_many_mut(&mut v, [0, 2, 4]);
*a += 10;
*b += 100;
*c += 1000;
assert_eq!(v, vec![11, 2, 103, 4, 1005]);
```
