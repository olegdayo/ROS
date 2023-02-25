#[macro_export]
macro_rules! c_strs {
    () => {
        Vec::<CString>::new()
    };

    ( $($cmd:expr) * ) => {
        {
            let mut temp_vec = Vec::<CString>::new();
            $(
                temp_vec.push(CString::new(std::stringify!($cmd)).unwrap());
            )*
            temp_vec
        }
    };
}
