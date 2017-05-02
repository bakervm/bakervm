mod cga_display;
mod console;

pub use self::cga_display::*;
pub use self::console::*;

use std::sync::mpsc::Sender;

pub trait Mountable {
    type DataFormat;

    fn run(&self) -> Sender<Self::DataFormat>;
}
