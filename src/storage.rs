/// `storage!` macro example
///
/// ```
/// #[macro_use]
/// extern crate localstorage_rs;
/// 
/// fn main() {
///     dotenv::dotenv().ok();
///     storage!(number, 1);
///     let value = get!(number);
///     println!("{}", value);
/// }
/// ```
///
/// This will store the number 1 under the key "number", and then retrieve it.


#[macro_export]
macro_rules! storage {
    ($name:tt, $value:tt) => {{
        use std::fs::File;
        use std::io::prelude::*;
        use std::env;
        let storage_path = env::var("STORAGE_PATH").expect("Storage directory path not found");
        let file_extension = env::var("FILE_EXTENSION").unwrap_or("txt".to_string());
        let mut file = File::create(format!("{}/{}.{}", storage_path, stringify!($name), file_extension)).expect("Unable_to_create_file");
        let value_str = format!("{}", $value);
        file.write_all(value_str.as_bytes()).expect("Unable to write data");
    }};
}

#[macro_export]
macro_rules! get {
    ($name:tt) => {{
        use std::fs::File;
        use std::io::prelude::*;
        use std::env;
        let storage_path = env::var("STORAGE_PATH").expect("Storage directory path not found");
        let file_extension = env::var("FILE_EXTENSION").unwrap_or("txt".to_string());
        let mut file = File::open(format!("{}/{}.{}", storage_path, stringify!($name), file_extension)).expect("Unable_to_open_file");        
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Unable to read data");
        contents
    }};
}


#[cfg(test)]
mod tests {
    use std::env;

    fn setup() {
        env::set_var("STORAGE_PATH", "/Users/solidoracle/Documents/Web3/Rust/localstorage-rs/storage");
        env::set_var("FILE_EXTENSION", "txt");
    }


    #[test]
    fn test_storage_and_get() {
        setup();

        // Test with string
        storage!(test_string, "Hello, world!");
        let value = get!(test_string);
        assert_eq!(value, "Hello, world!");

        // Test with integer
        storage!(test_int, 123);
        let value = get!(test_int);
        assert_eq!(value, "123");

        // Test with float
        storage!(test_float, 45.67);
        let value = get!(test_float);
        assert_eq!(value, "45.67");
    }
}