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

## Motivation

For `Vec`, `Into` is not implement for arrays greater than a size of 12.

```rust
let v: Vec<i32> = vec![0; 768];
let arr: [i32; 768] = v.into(); /// will not compile

let v: Vec<i32> = vec![0; 768];
let arr: Result<[i32; 768], _ > = v.try_into(); /// Will be an Err
```

Solution this crate adds `vec_to_array` and `try_vec_to_array`
