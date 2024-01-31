/// Macro to get string input from user
#[allow(unused_macros)]
macro_rules! get_string_value {
    ($input:expr) => {
        use std::io::{self, Write};
        stdin().read_line(&mut $input).expect("Not a valid string");
    };
}

/// Macro to get an integer number from user
#[allow(unused_macros)]
macro_rules! get_i32_value {
    () => {
        (|| {
            use std::io::{stdin};
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed to read line");
            match input.trim().parse::<i32>() {
                Ok(input) => input,
                Err(_) => {
                    println!("Failed to parse to integer");
                    return 0
                }
            }
        })()
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
