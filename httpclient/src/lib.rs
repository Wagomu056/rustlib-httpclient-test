use std::ffi::{c_char, c_uint, CStr, CString};
use std::thread;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{StatusCode};
use serde::{Serialize, Deserialize};
use futures::executor::block_on;

#[repr(C)]
pub struct HttpCallbackParam {
    user_id: i32,
    id: i32,
    title: *const c_char,
    completed: bool,
}

#[repr(C)]
pub struct PostRequestParam {
    user_id: u32,
    title: *const c_char,
    body: *const c_char,
}

#[derive(Serialize)]
struct PostRequestParamInternal {
    user_id: u32,
    title: String,
    body: String,
}

#[derive(Deserialize)]
struct PostResponse {
    user_id: u32,
    title: String,
    #[serde(rename = "body")]
    _body: String,
    id: u32,
}

#[derive(Deserialize)]
struct ResponseBody {
    #[serde(rename = "userId")]
    user_id: i32,
    id: i32,
    title: String,
    completed: bool,
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

type RequestCallback = extern "C" fn(bool, *const Post);

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
        match post {
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
    });
}

fn convert_post_param(param: &PostRequestParam) -> PostRequestParamInternal {
    let title = unsafe { CStr::from_ptr(param.title) };
    let title = title.to_str().unwrap().to_owned();

    let body = unsafe { CStr::from_ptr(param.body) };
    let body = body.to_str().unwrap().to_owned();

    PostRequestParamInternal {
        user_id: param.user_id,
        title,
        body,
    }
}

#[no_mangle]
pub extern fn post_request(param: &PostRequestParam, callback: extern "C" fn(bool, *const HttpCallbackParam)) {
    println!("create request");
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("application/json; charset=UTF-8"));
    let param = convert_post_param(&param);

    thread::spawn( move || {
        println!("post_request >>>>>");
        let client = reqwest::blocking::Client::new();
        let response = client.post("https://jsonplaceholder.typicode.com/posts")
            .headers(headers)
            .json(&param)
            .send();

        if response.is_err() {
            println!("Error {}", response.err().unwrap());
            callback(false, std::ptr::null_mut());
            return;
        }

        let response = response.unwrap();
        println!("Status: {}", &response.status());
        match response.status() {
            StatusCode::CREATED => {
                let body = response.text().unwrap();
                println!("Body:\n{}", &body);

                let body = serde_json::from_str(&body);
                if body.is_err() {
                    callback(false, std::ptr::null_mut());
                    return
                }

                let body : PostResponse = body.unwrap();
                let title = CString::new(body.title).unwrap();
                println!("title: {}", title.to_str().unwrap());
                let callback_param = HttpCallbackParam{
                    user_id: body.user_id as i32,
                    id: body.id as i32,
                    title: title.as_ptr(),
                    completed: true,
                };

                callback(true, &callback_param as *const HttpCallbackParam);
            }
            _ => {
                println!("Error {}", &response.text().unwrap());
                callback(false, std::ptr::null_mut());
            }
        }
        println!("<<<<< post_request end");
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
        static mut TITLE: String = String::new();

        extern "C" fn callback(is_success: bool, post: *const Post) {
            println!("callback is_success: {}", is_success);
            unsafe  {
                IS_RETURNED = true;
                IS_SUCCESS = is_success;
            }

            if is_success {
                unsafe {
                    ID = (*post).id;
                    TITLE = CStr::from_ptr((*post).title).to_str().unwrap().to_string();
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
            assert_eq!(TITLE, String::from("sunt aut facere repellat provident occaecati excepturi optio reprehenderit"));
        }
    }

    #[test]
    fn post_request_work() {
        static mut IS_RETURNED: bool = false;
        static mut IS_SUCCESS: bool = false;
        static mut USER_ID: i32 = 0;
        static mut TITLE: String = String::new();

        extern "C" fn post_request_callback(is_success: bool, callback_param: *const HttpCallbackParam) {
            println!("callback is_success: {}", is_success);
            unsafe  {
                IS_RETURNED = true;
                IS_SUCCESS = is_success;
            }

            if is_success {
                unsafe {
                    USER_ID = (*callback_param).user_id;
                    TITLE = CStr::from_ptr((*callback_param).title).to_str().unwrap().to_string();
                }
            }
        }

        println!("create param");
        let post_request_param = PostRequestParam{
            user_id: 123,
            title: CString::new("This is title.").unwrap().into_raw(),
            body: CString::new("Body message.").unwrap().into_raw(),
        };

        println!("prev call request");
        post_request(&post_request_param, post_request_callback);
        println!("post call request");

        unsafe {
            while IS_RETURNED == false {
            }
            assert_eq!(IS_SUCCESS, true);
            assert_eq!(USER_ID, 123);
            assert_eq!(TITLE, String::from("This is title."));
        }
    }
}
