extern crate cc;

fn main() {
    cc::Build::new()
        .file("sum_at&t.s")
        .file("sum_intel.s")
        .compile("asm-sum-lib");
}
