#![forbid(missing_docs)]
#![forbid(unsafe_code)]
#![warn(clippy::doc_markdown)]
#![doc = include_str!("../README.md")]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

pub mod orientation;
pub mod position;

/// The most commonly useful bits of the library
pub mod prelude {
    pub use crate::orientation::{Direction, Rotation};
    pub use crate::position::Position;
}
