use std::arch::asm;

extern "C" {
    fn asum(a: u64, b: u64) -> u64;
}

unsafe fn sum(first: u64, second: u64) -> u64 {
    let mut sum = 0u64;
    asm!(
        "add {sum}, {first}",
        "add {sum}, {second}",
        first = in(reg) first,
        second = in(reg) second,
        sum = out(reg) sum,
    );
    sum
}

fn main() {
    let a = 1u64;
    let b = 2u64;
    println!("a: {}", a);
    println!("b: {}", b);

    unsafe {
        println!("sum: {}", sum(a, b));
    }

    let a = 10u64;
    let b = 20u64;
    println!("a: {}", a);
    println!("b: {}", b);

    unsafe {
        println!("sum: {}", asum(a, b));
    }
}
