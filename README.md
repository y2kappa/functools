## Functools

Collection of functional utilities for Rust.

[![Crates.io][crates-badge]][crates-url]

[crates-badge]: https://img.shields.io/crates/v/ftools.svg
[crates-url]: https://crates.io/crates/ftools

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

Async pipe. Instead of:

```rust
let x = f(1).await;
let y = g(x).await;
let z = h(y).await;
```

you can do:
```rust
let x = pipe!(1 => f => g => h);
```