/// Macro for reverse a string
#[allow(unused_macros)]
macro_rules! reverse_string {
    ($s:expr) => {
        $s.chars().rev().collect::<String>()
    };
}