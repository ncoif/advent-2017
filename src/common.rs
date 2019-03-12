#[macro_export]
macro_rules! vec_of_strings {
    // match a list of expressions separated by comma:
    ($($str:expr),*) => ({
        // create a Vec with this list of expressions,
        // calling String::from on each:
        vec![$(String::from($str),)*] as Vec<String>
    });
}

pub fn to_char(s: &str) -> char {
    s.as_bytes()[0] as char
}
