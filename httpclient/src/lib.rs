use std::ffi::{c_char, CString};
use std::thread;

#[repr(C)]
pub struct HttpCallbackParam {
    name: *const c_char,
}

#[no_mangle]
pub extern fn http_request(callback: extern "C" fn(bool, *mut HttpCallbackParam)) {
    thread::spawn( move || {
        println!("http_request");
        let response = reqwest::blocking::get("https://www.example.com");
        match response {
            Ok(res) => {
                println!("Status: {}", res.status());
                println!("Body:\n{}", res.text().unwrap());

                let name = CString::new("Takuya").unwrap();
                let callback_param = HttpCallbackParam{
                    name: name.as_ptr(),
                };
                std::mem::forget(name);

                callback(true, Box::into_raw(Box::new(callback_param)));
                //callback(true, &callback_param as *const _ as *mut HttpCallbackParam);
            }
            Err(err) => {
                println!("Error: {}", err);
                callback(false, std::ptr::null_mut());
            }
        }
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

        extern "C" fn http_request_callback(is_success: bool, callback_param: *mut HttpCallbackParam) {
            unsafe {
                IS_RETURNED = true;
                IS_SUCCESS = is_success;
                NAME = CStr::from_ptr((*callback_param).name).to_str().unwrap();
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
            assert_eq!(NAME, "Takuya");
        }
    }
}
