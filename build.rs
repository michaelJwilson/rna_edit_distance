extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/base_frac.c")
        .compile("base_frac");
}