//! Moves a heap allocated `Vec<T>` to an stack allocated array of type `T` and size `N`.


/// Tries to move a `Vec<T>` into an array of type `T` and size `N`.
///
/// # Arguments
///
/// - `$vec`: The vector to be moved.
/// - `$t`: The type of the elements in the vector and array.
/// - `$size`: The expected size of the array.
///
/// # Returns
///
/// - `Ok([T; N])` if the vector can be moved successfully.
/// - `Err(VecToArrayError::SizeMismatch)` if the size of the vector doesn't match the specified size.
///
/// # Examples
///
/// ```
/// use vec_to_array::try_vec_to_array;
/// let v = vec![1, 2, 3];
/// let arr: [i32; 3] = try_vec_to_array!(v, i32, 3).unwrap();
/// assert_eq!(arr, [1, 2, 3]);
/// ```
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
                Err($crate::VecToArrayError::SizeMismatch { expected: $size, found: $vec.len() })
            }
        }
    }
}

/// Moves a `Vec<T>` into an array of type `T` and size `N`.
///
/// This macro will panic if the size of the vector is different from the specified size.
///
/// # Arguments
///
/// - `$vec`: The vector to be moved.
/// - `$t`: The type of the elements in the vector and array.
/// - `$size`: The expected size of the array.
///
/// # Panics
///
/// Panics if the size of the vector doesn't match the specified size.
///
/// # Examples
///
/// ```
/// use vec_to_array::vec_to_array;
/// let v = vec![1, 2, 3];
/// let arr: [i32; 3] = vec_to_array!(v, i32, 3);
/// assert_eq!(arr, [1, 2, 3]);
/// ```
#[macro_export]
macro_rules! vec_to_array {
    ($vec:ident, $t:ty, $size:expr) => {
        {
            if $vec.len() != $size {
                panic!("{}", $crate::VecToArrayError::SizeMismatch {
                    expected: $size,
                    found: $vec.len()
                });
            }
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

#[derive(Debug)]
pub enum VecToArrayError {
    SizeMismatch { expected: usize, found: usize },
}

impl std::fmt::Display for VecToArrayError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            VecToArrayError::SizeMismatch { expected, found } => write!(f, "Expected vector of size {}, but found size {}", expected, found),
            // Handle other error variants here if added.
        }
    }
}

impl std::error::Error for VecToArrayError {}


#[cfg(test)]
mod tests {
    use crate::VecToArrayError;

    #[test]
    fn basic_test() {
        let vec: Vec<i64> = vec![1, 2, 3];
        let array: [i64; 3] = vec_to_array!(vec, i64, 3);
        assert_eq!(array, [1, 2, 3]);
        let vec: Vec<i32> = vec![1, 2, 3];
        let array: Result<[i32; 3], VecToArrayError> = try_vec_to_array!(vec, i32, 3);
        assert_eq!(array.unwrap(), [1, 2, 3]);

        let v: Vec<i32> = vec![0; 768];
        let _arr: [i32; 768] = vec_to_array!(v, i32, 768);

        let v: Vec<i32> = Vec::with_capacity(768);
        let arr = try_vec_to_array!(v, i32, 768);
        match arr {
            Err(VecToArrayError::SizeMismatch { expected: 768, found: 0 }) => (),
            _ => panic!()
        }
        let v: Vec<i32> = vec![0; 768];
        let arr: Result<[i32; 768], VecToArrayError> = try_vec_to_array!(v, i32, 768);
        assert!(arr.is_ok());
    }

    #[test]
    fn more_complex_test() {
        let mut x: Vec<Vec<String>> = Vec::new();
        x.push(vec![String::from("12"), String::from("3")]);
        let exp = x.clone();
        let y: [Vec<String>; 1] = vec_to_array!(x, Vec<String>, 1);
        let act = y.to_vec();
        assert_eq!(act, exp);
    }

    #[test]
    fn test_will_not_panic() {
        let v: Vec<i32> = vec![0; 768];
        let _arr: Result<[i32; 769], VecToArrayError> = try_vec_to_array!(v, i32, 769);
        let v: Vec<i32> = vec![0; 768];
        let _arr: Result<[i32; 767], VecToArrayError> = try_vec_to_array!(v, i32, 767);
    }

    #[test]
    #[should_panic]
    fn test_panics1() {
        let v: Vec<i32> = vec![0; 768];
        let _arr: [i32; 767] = vec_to_array!(v, i32, 767);
    }

    #[test]
    #[should_panic]
    fn test_panics2() {
        let v: Vec<i32> = vec![0; 768];
        let _arr: [i32; 769] = vec_to_array!(v, i32, 769);
    }

    #[test]
    #[should_panic]
    fn test_panics3() {
        let v: Vec<i32> = Vec::with_capacity(768);
        let _arr: [i32; 768] = vec_to_array!(v, i32, 768);
    }
}
