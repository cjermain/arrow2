mod basic;
mod nested;

pub(crate) use basic::build_statistics;
pub use basic::array_to_page;
pub(crate) use basic::encode_plain;
pub use nested::array_to_page as nested_array_to_page;
