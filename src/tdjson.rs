//! Interface for methods defined in [td/td_json_client.h](https://github.com/tdlib/td/blob/master/td/telegram/td_json_client.h).
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::{c_char, c_double, c_int, c_long};
use std::ptr;

pub type ClientId = i32;
#[link(name = "tdjson")]
extern "C" {
    fn td_create_client_id() -> c_int;
    fn td_send(client_id: c_int, request: *const c_char);
    fn td_receive(timeout: c_double) -> *const c_char;
    fn td_execute(request: *const c_char) -> *const c_char;

    // Deprecated. Use setLogVerbosityLevel request instead.
    fn td_set_log_verbosity_level(level: c_int);
    // Deprecated. Use setLogStream request instead.
    fn td_set_log_file_path(path: *const c_char) -> c_int;
    // Deprecated. Use setLogStream request instead.
    fn td_set_log_max_file_size(size: c_long);
}

pub fn new_client() -> ClientId {
    unsafe { td_create_client_id() }
}

pub fn send(client_id: ClientId, request: &str) {
    let cstring = CString::new(request).unwrap();
    unsafe { td_send(client_id, cstring.as_ptr()) }
}

pub fn execute(request: &str) -> Option<String> {
    let cstring = CString::new(request).unwrap();
    let result = unsafe {
        td_execute(cstring.as_ptr())
            .as_ref()
            .map(|response| CStr::from_ptr(response).to_string_lossy().into_owned())
    };
    result
}

pub fn receive(timeout: f64) -> Option<String> {
    unsafe {
        td_receive(timeout)
            .as_ref()
            .map(|response| CStr::from_ptr(response).to_string_lossy().into_owned())
    }
}

// Deprecated. Use setLogVerbosityLevel request instead.
pub fn set_log_verbosity_level(level: i32) {
    unsafe { td_set_log_verbosity_level(level) };
}

// Deprecated. Use setLogStream request instead.
pub fn set_log_file_path(path: Option<&str>) -> bool {
    let result = match path {
        None => unsafe { td_set_log_file_path(ptr::null()) },
        Some(path_) => {
            let cpath = CString::new(path_).unwrap();
            unsafe { td_set_log_file_path(cpath.as_ptr()) }
        }
    };
    match result {
        1 => true,
        0 => false,
        _ => panic!("unexpected response from libtdjson: {:?}", result),
    }
}

// Deprecated. Use setLogStream request instead.
pub fn set_log_max_file_size(size: i64) {
    unsafe { td_set_log_max_file_size(size as c_long) };
}
