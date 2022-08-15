#[macro_export]
macro_rules! some_or_continue {
    ($thing:expr) => {
        match $thing {
            Some(v) => v,
            None => continue,
        }
    };
}
