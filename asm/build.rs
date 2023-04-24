extern crate cc;

#[cfg(target_os = "linux")]
fn main() {
    cc::Build::new()
        .file("asm/linux/sum_at&t.s")
        .file("asm/linux/sum_intel.s")
        .compile("asm-sum-lib");
}

#[cfg(target_os = "macos")]
fn main() {
    cc::Build::new()
        .file("asm/macos/sum_at&t.s")
        .file("asm/macos/sum_intel.s")
        .compile("asm-sum-lib");
}
