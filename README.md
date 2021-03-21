## Functools

Collection of functional utilities for Rust.

```rust
let xs = vec![1, 2, 3];
let ys = vec![4, 5, 6];

assert_eq!(vector::zip_with(xs, ys, |x, y| x + y), vec![5, 7, 9]);
```

```rust
let xs = vec![1, 3, 5, 7, 9];
let is_odd = |x| x % 2 != 0;

assert_eq!(truth::all(&xs, is_odd), true)
```