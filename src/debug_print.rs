#[macro_export]
macro_rules! debug_msg {
    ($($arg:tt)*) => {
        if cfg!(feature = "debug-msg-print") {
            println!($($arg)*);
        }
    };
}
