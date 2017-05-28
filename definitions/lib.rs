#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate bincode;

pub mod typedef;
pub mod program;
pub mod image_builder;
pub mod value;
pub mod config;

pub use value::Value;
