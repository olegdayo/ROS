#[macro_export]
macro_rules! c_strs {
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
                if &elem == "." {
                    if res_vec.len() == 0 {
                        res_vec.push(elem);
                        continue;
                    }

                    *res_vec.last_mut().unwrap() += &elem;
                    continue;
                }

                match res_vec.last() {
                    Some(cmd) => {
                        match *cmd.as_bytes().last().unwrap() as char {
                            '-' | '.' => {
                                *res_vec.last_mut().unwrap() += &elem;
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

            res_vec.iter().map(|s: &String| CString::new(s.clone()).unwrap()).collect::<Vec<CString>>()
        }
    };
}
