/// tty2cstr macro converts linux terminal command into vector of C strings
///
/// Failures:
///
/// ls .vscode => ["ls.vscode"]
///
/// git push --set-upstream origin master => ["git", "push", "--set", "-upstream", "origin", "master"]
#[macro_export]
macro_rules! tty2cstr {
    () => {
        Vec::<CString>::new()
    };

    ( $($cmd:tt)* ) => {
        {
            let mut temp_vec = Vec::<String>::new();
            $(
                temp_vec.push(std::stringify!($cmd).to_string());
            )*

            let mut res_vec = Vec::<String>::new();
            for elem in temp_vec {
                match elem.as_str() {
                    "." | "," | ";" => {
                        match res_vec.last_mut() {
                            Some(last) => {
                                *last += &elem;
                            }

                            None => {
                                res_vec.push(elem);
                            }
                        }

                        continue;
                    }
                    _ => {}
                }

                match res_vec.last_mut() {
                    Some(last) => {
                        match *last.as_bytes().last().unwrap() as char {
                            '-' | '.' | ',' | ';' => {
                                *last += &elem;
                            }
                            _ => {
                                res_vec.push(elem);
                            }
                        }
                    }

                    None => {
                        res_vec.push(elem);
                    }
                }
            }

            res_vec.iter().map(|s: &String| CString::new(s.as_str()).unwrap()).collect::<Vec<CString>>()
        }
    };
}
