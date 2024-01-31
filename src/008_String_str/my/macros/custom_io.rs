/// Macro to get string input from user
#[allow(unused_macros)]
macro_rules! get_string_value {
    ($input:expr) => {
        use std::io::{stdin, self, Write};
        stdin().read_line(&mut $input).expect("Not a valid string");
    };
}

/// Macro for waiting a key pressed from user
#[allow(unused_macros)]
macro_rules! wait_for_key_press {
    () => {
        println!("\nPress any key to continue...");
        stdin().read_line(&mut String::new()).expect("Failed to read");
    };
}

/// Macro for flush stdout
#[allow(unused_macros)]
macro_rules! flush_stdout {
    () => {
        let _ = io::stdout().flush();
    };
}
