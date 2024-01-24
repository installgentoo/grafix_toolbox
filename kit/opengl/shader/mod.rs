pub use shader::*;

#[macro_use]
pub mod shader;
#[macro_use]
pub mod uniform;

mod args;
mod parsing;

use {super::internal::*, crate::lib::*};
