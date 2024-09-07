# vec_to_array

Moves a heap allocated `Vec` into a stack allocated array.

```rust
let vec: Vec<i64> = vec![1, 2, 3];
let array: [i64; 3] = vec_to_array!(vec, i64, 3);
assert_eq!(array, [1, 2, 3]);

let vec: Vec<i32> = vec![1, 2, 3];
let array: Result<[i32; 3], VecToArrayError> = try_vec_to_array!(vec, i32, 3);
assert_eq!(array.unwrap(), [1, 2, 3]);
```

In most cases you will want to prefer using `try_into`.
(https://doc.rust-lang.org/alloc/vec/struct.Vec.html#impl-TryFrom%3CVec%3CT,+A%3E%3E-for-%5BT;+N%5D).