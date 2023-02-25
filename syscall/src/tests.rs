use crate::c_strs;

use std::ffi::CString;


#[test]
fn empty_terminal() {
    assert_eq!(Vec::<CString>::new(), c_strs![])
}

#[test]
fn full_terminal() {
    let expected = vec![
        CString::new("less").unwrap(),
        CString::new("access.log").unwrap(),
        CString::new("|").unwrap(),
        CString::new("grep").unwrap(),
        CString::new("-E").unwrap(),
        CString::new("\"(WARN|ERROR|FATAL)\"").unwrap(),
        CString::new("|").unwrap(),
        CString::new("awk").unwrap(),
        CString::new("\"{print($2)}\"").unwrap(),
        CString::new("|").unwrap(),
        CString::new("uniq").unwrap(),
        CString::new("-c").unwrap(),
        CString::new("|").unwrap(),
        CString::new("sort").unwrap(),
        CString::new("-nrk1").unwrap(),
    ];
    assert_eq!(
        expected,
        c_strs![less access.log | grep -E "(WARN|ERROR|FATAL)" | awk "{print($2)}" | uniq -c | sort -nrk1],
    )
}

#[test]
fn spaces() {
    let expected = vec![
        CString::new("less").unwrap(),
        CString::new("access.log").unwrap(),
        CString::new("|").unwrap(),
        CString::new("grep").unwrap(),
        CString::new("-E").unwrap(),
        CString::new("\"(WARN|ERROR|FATAL)\"").unwrap(),
        CString::new("|").unwrap(),
        CString::new("awk").unwrap(),
        CString::new("\"{print $2}\"").unwrap(),
        CString::new("|").unwrap(),
        CString::new("uniq").unwrap(),
        CString::new("-c").unwrap(),
        CString::new("|").unwrap(),
        CString::new("sort").unwrap(),
        CString::new("-nrk1").unwrap(),
    ];
    assert_eq!(
        expected,
        c_strs![less access.log | grep -E "(WARN|ERROR|FATAL)" | awk "{print $2}" | uniq -c | sort -nrk1],
    )
}
