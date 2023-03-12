use std::ffi::c_char;
use std::thread;

#[no_mangle]
pub extern fn rust_hello() -> *mut c_char {
    let hello = "Hello iPhone from Rust";
    let c_string = std::ffi::CString::new(hello).unwrap();
    c_string.into_raw()
}

#[no_mangle]
pub extern fn http_request(callback: extern "C" fn(bool)) {
    thread::spawn( move || {
        println!("http_request");
        let response = reqwest::blocking::get("https://www.example.com");
        match response {
            Ok(res) => {
                println!("Status: {}", res.status());
                println!("Body:\n{}", res.text().unwrap());
                callback(true);
            }
            Err(err) => {
                println!("Error: {}", err);
                callback(false);
            }
        }
    });
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


    #[test]
    fn http_request_works() {
        static mut IS_RETURNED: bool = false;
        static mut IS_SUCCESS: bool = false;

        extern "C" fn http_request_callback(is_success: bool) {
            unsafe {
                IS_RETURNED = true;
                IS_SUCCESS = is_success;
            }
            println!("is_success: {}", is_success);
        }

        println!("prev request");
        http_request(http_request_callback);
        println!("post request");

        unsafe {
            while IS_RETURNED == false {
            }
            assert_eq!(IS_SUCCESS, true);
        }
    }
}
