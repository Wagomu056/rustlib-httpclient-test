use std::ffi::{c_char, c_uint, CStr, CString};
use std::thread;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{StatusCode};
use serde::{Serialize, Deserialize};
use futures::executor::block_on;

type RequestCallback = extern "C" fn(bool, *const Post);

#[repr(C)]
pub struct RequestPost {
    user_id: c_uint,
    title: *const c_char,
    body: *const c_char,
}

#[derive(Serialize)]
struct RequestPostImpl {
    #[serde(rename = "userId")]
    user_id: c_uint,
    title: String,
    body: String,
}

impl RequestPostImpl {
    pub fn init(param: &RequestPost) -> RequestPostImpl {
        let title = unsafe { CStr::from_ptr(param.title) };
        let title = title.to_str().unwrap().to_owned();

        let body = unsafe { CStr::from_ptr(param.body) };
        let body = body.to_str().unwrap().to_owned();

        RequestPostImpl {
            user_id: param.user_id,
            title,
            body,
        }
    }
}

#[repr(C)]
pub struct Post {
    id: c_uint,
    user_id: c_uint,
    title: *const c_char,
    body: *const c_char,
}

#[derive(Debug, Deserialize)]
struct PostImpl {
    id: c_uint,
    #[serde(rename = "userId")]
    user_id: c_uint,
    title: String,
    body: String,
}

fn call_callback_by_pos_impl(post_impl: Option<PostImpl>, callback: RequestCallback) {
    match post_impl {
        Some(p) => {
            let title = CString::new(p.title).unwrap();
            let body = CString::new(p.body).unwrap();
            let post = Post{
                user_id: p.user_id,
                id: p.id,
                title: title.as_ptr(),
                body: body.as_ptr(),
            };
            callback(true, &post as *const Post);
        }
        None => {
            callback(false, std::ptr::null_mut());
        }
    }
}

// ----------
// Get
// ----------
async fn get_request_impl() -> Option<PostImpl> {
    println!("get_request_impl >>>>>");
    let client = reqwest::blocking::Client::new();
    let res = client.get("https://jsonplaceholder.typicode.com/posts/1")
        .send()
        .map_err(|e| println!("network error: {}", e))
        .ok()?;

    let status = res.status();
    println!("Status: {}", &status);

    let body = res.text()
        .map_err(|e| println!("get text error: {}", e))
        .ok()?;
    println!("Body:\n{}", &body);

    let post = serde_json::from_str::<PostImpl>(&body)
        .map_err(|e| println!("json error: {}", e))
        .ok();
    println!("<<<<< get_request_impl end");

    post
}

#[no_mangle]
pub extern fn get_request(callback: RequestCallback) {
    thread::spawn( move || {
        let post = block_on(get_request_impl());
        call_callback_by_pos_impl(post, callback);
    });
}

// ----------
// Post
// ----------
async fn post_request_impl(param: &RequestPostImpl) -> Option<PostImpl> {
    println!("post_request_impl >>>>>");
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("application/json; charset=UTF-8"));

    let client = reqwest::blocking::Client::new();
    let response = client.post("https://jsonplaceholder.typicode.com/posts")
        .headers(headers)
        .json(&param)
        .send()
        .ok()?;

    println!("Status: {}", &response.status());
    match response.status() {
        StatusCode::CREATED => {
            let body = response.text().ok()?;
            println!("Body:\n{}", &body);
            let post = serde_json::from_str::<PostImpl>(&body)
                .map_err(|e| println!("json error: {}", e))
                .ok();
            println!("<<<<< post_request end");
            post
        }
        _ => {
            println!("Error {}", &response.status().to_string());
            println!("<<<<< post_request end");
            None
        }
    }
}

#[no_mangle]
pub extern fn post_request(param: &RequestPost, callback: RequestCallback) {
    let param = RequestPostImpl::init(&param);
    thread::spawn( move || {
        let post = block_on(post_request_impl(&param));
        call_callback_by_pos_impl(post, callback);
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
        static mut ID: u32 = 0;
        static mut USER_ID: u32 = 0;
        static mut TITLE: String = String::new();
        static mut BODY: String = String::new();

        extern "C" fn callback(is_success: bool, post: *const Post) {
            println!("callback is_success: {}", is_success);
            unsafe  {
                IS_RETURNED = true;
                IS_SUCCESS = is_success;
            }

            if is_success {
                unsafe {
                    ID = (*post).id;
                    USER_ID = (*post).user_id;
                    TITLE = CStr::from_ptr((*post).title).to_str().unwrap().to_string();
                    BODY = CStr::from_ptr((*post).body).to_str().unwrap().to_string();
                }
            }
        }

        println!("prev call request");
        get_request(callback);
        println!("post call request");

        unsafe {
            while IS_RETURNED == false {
            }
            assert_eq!(IS_SUCCESS, true);
            assert_eq!(ID, 1);
            assert_eq!(USER_ID, 1);
            assert_eq!(TITLE, String::from("sunt aut facere repellat provident occaecati excepturi optio reprehenderit"));
            assert_eq!(BODY, String::from("quia et suscipit\nsuscipit recusandae consequuntur expedita et cum\nreprehenderit molestiae ut ut quas totam\nnostrum rerum est autem sunt rem eveniet architecto"));
        }
    }

    #[test]
    fn post_request_work() {
        static mut IS_RETURNED: bool = false;
        static mut IS_SUCCESS: bool = false;
        static mut ID: u32 = 0;
        static mut USER_ID: u32 = 0;
        static mut TITLE: String = String::new();
        static mut BODY: String = String::new();

        extern "C" fn callback(is_success: bool, post: *const Post) {
            println!("callback is_success: {}", is_success);
            unsafe  {
                IS_RETURNED = true;
                IS_SUCCESS = is_success;
            }

            if is_success {
                unsafe {
                    ID = (*post).id;
                    USER_ID = (*post).user_id;
                    TITLE = CStr::from_ptr((*post).title).to_str().unwrap().to_string();
                    BODY = CStr::from_ptr((*post).body).to_str().unwrap().to_string();
                }
            }
        }

        println!("create param");
        let post_request_param = RequestPost {
            user_id: 123,
            title: CString::new("This is title.").unwrap().into_raw(),
            body: CString::new("Body message.").unwrap().into_raw(),
        };

        println!("prev call request");
        post_request(&post_request_param, callback);
        println!("post call request");

        unsafe {
            while IS_RETURNED == false {
            }
            assert_eq!(IS_SUCCESS, true);
            assert_eq!(ID, 101);
            assert_eq!(USER_ID, 123);
            assert_eq!(TITLE, String::from("This is title."));
            assert_eq!(BODY, String::from("Body message."));
        }
    }
}
