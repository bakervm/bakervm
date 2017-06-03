//! All common type definitions through the bakerVM ecosystem
//!
//! # Example
//! ```
//! use definitions::typedef::*;
//! ```

pub type Byte = u8;
pub type Float = f64;
pub type Integer = i64;
pub type Address = usize;
pub type ImageData = Vec<Byte>;
pub type Color = (u8, u8, u8);
pub type Frame = Vec<Color>;
