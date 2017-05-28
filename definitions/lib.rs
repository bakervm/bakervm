#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate bincode;

pub mod config;
pub mod image_builder;
pub mod instruction;
pub mod interrupt;
pub mod program;
pub mod target;
pub mod typedef;
pub mod value;

pub use value::Value;

pub const PREAMBLE: &str = "BAKERVM";
