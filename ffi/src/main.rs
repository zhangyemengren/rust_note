use ffi::printInteger;
fn main() {
    unsafe {
        printInteger(32);
    }
    println!("Hello, world!");
}