use std::ffi::{c_char, CString};
use std::thread;

#[repr(C)]
pub struct HttpCallbackParam {
    name: *const c_char,
}

#[no_mangle]
pub extern fn http_request(callback: extern "C" fn(bool, *const HttpCallbackParam)) {
    thread::spawn( move || {
        println!("http_request >>>>>");
        let response = reqwest::blocking::get("https://www.example.com");
        match response {
            Ok(res) => {
                println!("Status: {}", res.status());
                println!("Body:\n{}", res.text().unwrap());

                let name = Box::new(CString::new("Takuya").unwrap());
                println!("name: {}", name.to_str().unwrap());
                let callback_param = HttpCallbackParam{
                    name: name.into_raw(),
                };

                callback(true, &callback_param as *const HttpCallbackParam);
            }
            Err(err) => {
                println!("Error: {}", err);
                callback(false, std::ptr::null_mut());
            }
        }
        println!("<<<<< http_request end");
        // jsonでパラメータを取得して、一部を構造体として返すのをやってみる。
        // それが終わったらPOSTも試してみる
    });
}

#[cfg(test)]
mod tests {
    use std::ffi::CStr;
    use super::*;

    #[test]
    fn http_request_works() {
        static mut IS_RETURNED: bool = false;
        static mut IS_SUCCESS: bool = false;
        static mut NAME: &str = "";

        extern "C" fn http_request_callback(is_success: bool, callback_param: *const HttpCallbackParam) {
            println!("callback is_success: {}", is_success);
            unsafe {
                IS_RETURNED = true;
                IS_SUCCESS = is_success;
                NAME = CStr::from_ptr((*callback_param).name).to_str().unwrap();
            }
        }

        println!("prev call request");
        http_request(http_request_callback);
        println!("post call request");

        unsafe {
            while IS_RETURNED == false {
            }
            assert_eq!(IS_SUCCESS, true);
            assert_eq!(NAME, "Takuya");
        }
    }
}
