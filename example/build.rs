extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/ctt.c")
        .compile("libctt.a");
}