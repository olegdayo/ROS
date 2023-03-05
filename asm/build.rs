extern crate cc;

fn main() {
    cc::Build::new()
        .file("sum.s")
        .compile("asm-lib");
}
