#![forbid(missing_docs)]
#![forbid(unsafe_code)]
#![warn(clippy::doc_markdown)]
#![doc = include_str!("../README.md")]

pub mod orientation;

/// The most commonly useful bits of the library
pub mod prelude {
    pub use {crate::orientation::Direction, crate::orientation::Rotation};
}
