#![forbid(missing_docs)]
#![forbid(unsafe_code)]
#![warn(clippy::doc_markdown)]
#![doc = include_str!("../README.md")]

pub mod orientation;
pub mod position;

/// The most commonly useful bits of the library
pub mod prelude {
    pub use crate::orientation::{Direction, Rotation};
    pub use crate::position::Position;
}
