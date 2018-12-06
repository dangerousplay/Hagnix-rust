extern crate libc;

use std::c_str::CString;
use std::os::raw::c_char;
use std::ffi::CString;

#[no_mangle]
pub extern "C" fn init(host: *const c_char, port: i64) ->  {
    let c_value = unsafe { CString::new(host)};
}

#[no_mangle]
pub extern "C" fn stop(){

}

#[no_mangle]
pub extern "C" fn stop(){

}