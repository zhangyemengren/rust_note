use std::ffi::{c_char, CString};

extern "C" {
    fn printInteger(x: i32);
    fn printString(x: *mut c_char);
}

pub fn call() {
    unsafe {
        printInteger(42);
        let s = CString::new("printString from Rust").unwrap();
        printString(s.as_ptr() as *mut _);
    }
}
fn main() {
    call();
}
