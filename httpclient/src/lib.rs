use std::ffi::c_char;

#[no_mangle]
pub extern fn rust_hello() -> *mut c_char {
    let hello = "Hello iPhone from Rust";
    let c_string = std::ffi::CString::new(hello).unwrap();
    c_string.into_raw()
}

#[cfg(test)]
mod tests {
    use std::ffi::CString;
    use super::*;

    #[test]
    fn it_works() {
        let c_str =
            unsafe {
                CString::from_raw(rust_hello())
            };
        let str = c_str.to_str().unwrap();
        assert_eq!(str, "Hello iPhone from Rust");
    }
}
