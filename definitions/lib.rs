#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate bincode;

mod config;
mod image_builder;
mod instruction;
mod interrupt;
mod program;
mod target;
mod value;
pub mod typedef;

pub use config::*;
pub use image_builder::*;
pub use instruction::*;
pub use interrupt::*;
pub use program::*;
pub use target::*;
pub use value::*;

pub const PREAMBLE: &str = "BAKERVM";
