use std::ffi::{c_char, CString};
use std::thread;
use serde::Deserialize;

#[repr(C)]
pub struct HttpCallbackParam {
    id: i32,
    title: *const c_char,
}

#[derive(Deserialize)]
struct ResponseBody {
    #[serde(rename = "userId")]
    _user_id: i32,
    id: i32,
    title: String,
    #[serde(rename = "completed")]
    _completed: bool,
}

#[no_mangle]
pub extern fn http_request(callback: extern "C" fn(bool, *const HttpCallbackParam)) {
    thread::spawn( move || {
        println!("http_request >>>>>");
        let response = reqwest::blocking::get("https://jsonplaceholder.typicode.com/todos/1");
        match response {
            Ok(res) => {
                println!("Status: {}", &res.status());

                let body_str = res.text().unwrap();
                println!("Body:\n{}", &body_str);
                let body : ResponseBody = serde_json::from_str(&body_str).unwrap();

                let title = CString::new(body.title).unwrap();
                println!("title: {}", title.to_str().unwrap());
                let callback_param = HttpCallbackParam{
                    id: body.id,
                    title: title.as_ptr(),
                };

                callback(true, &callback_param as *const HttpCallbackParam);
            }
            Err(err) => {
                println!("Error: {}", err);
                callback(false, std::ptr::null_mut());
            }
        }
        println!("<<<<< http_request end");
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
        static mut ID: i32 = 0;
        static mut TITLE: String = String::new();

        extern "C" fn http_request_callback(is_success: bool, callback_param: *const HttpCallbackParam) {
            println!("callback is_success: {}", is_success);
            unsafe {
                IS_RETURNED = true;
                IS_SUCCESS = is_success;
                ID = (*callback_param).id;
                TITLE = CStr::from_ptr((*callback_param).title).to_str().unwrap().to_string();
            }
        }

        println!("prev call request");
        http_request(http_request_callback);
        println!("post call request");

        unsafe {
            while IS_RETURNED == false {
            }
            assert_eq!(IS_SUCCESS, true);
            assert_eq!(ID, 1);
            assert_eq!(TITLE, String::from("delectus aut autem"));
        }
    }
}
