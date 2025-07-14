pub mod date;
pub mod dir;
pub mod iter;
pub mod wrapping_usize;

#[macro_export]
macro_rules! unwrap_or_ret {
    ($option:expr) => {
        match $option {
            Some(val) => val,
            _ => return,
        }
    };
}
