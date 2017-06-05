//! A crate for defining the core of the bakerVM

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate bincode;
extern crate regex;
#[macro_use]
extern crate lazy_static;

mod config;
mod image_builder;
mod instruction;
mod interrupt;
mod program;
mod target;
mod value;
mod type_t;
pub mod typedef;

pub use config::*;
pub use image_builder::*;
pub use instruction::*;
pub use interrupt::*;
pub use program::*;
pub use target::*;
pub use type_t::*;
pub use value::*;

pub const PREAMBLE: &str = "BAKERVM";
