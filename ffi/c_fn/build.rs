// build.rs

fn main() {
    cc::Build::new()
        .file("./third_party/src/fn_a.c")
        .file("./third_party/src/fn_b.c")
        .compile("mylib_cc");
}