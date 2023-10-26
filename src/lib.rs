//#![no_std]

/// Tries to transmute a [Vec] vec to an array of type t and size size. Returns a [Result].
#[macro_export]
macro_rules! try_vec_to_array {
    ($vec:ident, $t:ty, $size:expr) => {
        {
            if $vec.len() == $size {
                let mut arr: [std::mem::MaybeUninit<$t>; $size] = unsafe {
                    std::mem::MaybeUninit::uninit().assume_init()
                };
                for (i, item) in $vec.into_iter().enumerate() {
                    arr[i] =  std::mem::MaybeUninit::new(item);
                }
                Ok(unsafe { std::mem::transmute::<_, [$t; $size]>(arr) })
            } else {
                Err(format!("Expected vector of size {}, but found size {}", $size, $vec.len()))
            }
        }
    }
}

#[macro_export]
macro_rules! vec_to_array {
    ($vec:ident, $t:ty, $size:expr) => {
        {
                let mut arr: [std::mem::MaybeUninit<$t>; $size] = unsafe {
                    std::mem::MaybeUninit::uninit().assume_init()
                };
                for (i, item) in $vec.into_iter().enumerate() {
                    arr[i] =  std::mem::MaybeUninit::new(item);
                }
                unsafe { std::mem::transmute::<_, [$t; $size]>(arr) }
            }
        }
}


#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let vec = vec![1, 2, 3];
        let array = try_vec_to_array!(vec, i32, 3).unwrap();
        assert_eq!(array, [1, 2, 3]);
        let vec = vec![1, 2, 3];
        let array = vec_to_array!(vec, i64, 3);
        assert_eq!(array, [1, 2, 3]);
    }
}
