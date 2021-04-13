/// HashMap宏
/// # 例子
/// ```rust
/// let mut expected = HashMap::with_capacity(16);
/// expected.insert("hello", "world");
/// let product = hash_map!("hello" => "world");
/// assert_eq!(expected, product);
/// ```
#[macro_export]
macro_rules! hash_map {
    () => { std::collections::HashMap::with_capacity(16) };

    ($key: expr => $value: expr) => {
        // follow Java's HashMap.
        hash_map!($key => $value; 16)
    };
    ($key: expr => $value: expr; $init_capacity: expr) => {
        {
            let mut hash_map = std::collections::HashMap::with_capacity($init_capacity);
            hash_map.insert($key, $value);
            hash_map
        }
    };

    ($($key: expr => $value: expr),*) => {
        vec![$(($key, $value)),*].into_iter().collect::<std::collections::HashMap<_, _>>()
    };
    ($($key: expr => $value: expr,)*) => {
        hash_map!($($key => $value),*)
    };
}
