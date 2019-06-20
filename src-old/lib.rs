//! symrs - As symbolic mathematics libary for Rust.

#![feature(box_syntax)]
#![feature(core_intrinsics)]

// Included Modules
pub mod core;
pub mod errors;
pub mod expressions;

// Re-export common modules.
pub use crate::core::Expression;
pub use crate::errors::*;
pub use crate::expressions::irreducibles::*;
pub use crate::expressions::operations::elementary::*;
