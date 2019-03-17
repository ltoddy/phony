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
        {
            let mut hash_map = hash_map!();
            $(hash_map.insert($key, $value);)*
            hash_map
        }
    };
    ($($key: expr => $value: expr,)*) => {
        hash_map!($($key => $value),*)
    };
}
