use crate::{sum, asum};

static MAX_NUM: u64 = 100;

#[test]
fn check_sum() {
    for i in 0..MAX_NUM {
        for j in 0..MAX_NUM {
            unsafe {
                assert!(i + j == sum(i, j));
            }
        }
    }
}

#[test]
fn check_asum() {
    for i in 0..MAX_NUM {
        for j in 0..MAX_NUM {
            unsafe {
                assert!(i + j == asum(i, j));
            }
        }
    }
}
