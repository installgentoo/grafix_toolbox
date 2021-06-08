#[allow(unused_attributes)]
#[macro_export]
pub mod ext;
#[macro_use]
pub mod profiling;

pub mod cached_str;
pub mod prefetch;
pub mod slicing;
pub mod tuple;
