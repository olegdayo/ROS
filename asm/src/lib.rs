#[cfg(test)]
mod tests;

use std::arch::asm;

extern "C" {
    pub fn asum(a: u64, b: u64) -> u64;
}

pub unsafe fn sum(first: u64, second: u64) -> u64 {
    let mut sum: u64;
    asm!(
        "add {sum}, {first}",
        "add {sum}, {second}",
        first = in(reg) first,
        second = in(reg) second,
        sum = out(reg) sum,
    );
    sum
}
