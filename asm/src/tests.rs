use crate::{sum, aasum, aisum};

static MIN_NUM: u64 = 0;
static MAX_NUM: u64 = 100;

#[test]
fn check_sum() {
    for i in MIN_NUM..MAX_NUM {
        for j in MIN_NUM..MAX_NUM {
            unsafe {
                println!("{}, {}", i + j, sum(i, j));
                assert!(i + j == sum(i, j));
            }
        }
    }
}

#[test]
fn check_aasum() {
    for i in MIN_NUM..MAX_NUM {
        for j in MIN_NUM..MAX_NUM {
            unsafe {
                assert!(i + j == aasum(i, j));
            }
        }
    }
}

#[test]
fn check_aisum() {
    for i in MIN_NUM..MAX_NUM {
        for j in MIN_NUM..MAX_NUM {
            unsafe {
                assert!(i + j == aisum(i, j));
            }
        }
    }
}
