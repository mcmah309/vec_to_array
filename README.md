# vec_to_array

Moves a heap allocated `Vec` into a stack allocated array. In most cases you will want to prefer using `try_into`
(https://doc.rust-lang.org/alloc/vec/struct.Vec.html#impl-TryFrom%3CVec%3CT,+A%3E%3E-for-%5BT;+N%5D)
unless you need the array on stack for some reason.

```rust
let vec: Vec<i64> = vec![1, 2, 3];
let array: [i64; 3] = vec_to_array!(vec, i64, 3);
assert_eq!(array, [1, 2, 3]);

let vec: Vec<i32> = vec![1, 2, 3];
let array: Result<[i32; 3], VecToArrayError> = try_vec_to_array!(vec, i32, 3);
assert_eq!(array.unwrap(), [1, 2, 3]);
```

## Motivation

For `Vec`, `Into` is not implement for arrays greater than a size of 12.

```rust
let v: Vec<i32> = vec![0; 768];
let arr: [i32; 768] = v.into(); /// will not compile

let v: Vec<i32> = vec![0; 768];
let arr: Result<[i32; 768], _ > = v.try_into(); /// Will work but is on the heap
```

Solution this crate adds `vec_to_array!` and `try_vec_to_array!`
