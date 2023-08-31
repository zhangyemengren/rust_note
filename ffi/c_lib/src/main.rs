use c_lib::{printInteger, printString};
use std::ffi::{CString};
fn main() {
    unsafe {
        let input_string = "Hello, FFI!";
        let c_string = CString::new(input_string).expect("CString creation failed");
        printInteger(32);
        printString(c_string.as_ptr() as *mut _);
    }
    println!("Hello, world!");
}