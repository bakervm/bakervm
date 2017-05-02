use definitions::typedef::*;
use output::Mountable;
use std::sync::mpsc::{self, Sender};
use std::thread;

// We use CGA for the display resolution
const DISPLAY_WIDTH: usize = 320;
const DISPLAY_HEIGHT: usize = 200;

/// A register for displaying color data on a virtual display
pub struct DisplayData {
    color_mode: ColorMode,
    data: [[SmallWord; DISPLAY_HEIGHT]; DISPLAY_WIDTH],
}

impl DisplayData {
    pub fn new() -> DisplayData {
        DisplayData {
            color_mode: ColorMode::_24Bit,
            data: [[0; DISPLAY_HEIGHT]; DISPLAY_WIDTH],
        }
    }
}

/// The mode for *interpreting* the color data in the framebuffer
pub enum ColorMode {
    _1Bit,
    _8Bit,
    _24Bit,
}

/// A virtual CGA Display
pub struct CGADisplay {}

impl Mountable for CGADisplay {
    type DataFormat = DisplayData;

    fn run(&self) -> Sender<Self::DataFormat> {
        let (sender, receiver) = mpsc::channel::<Self::DataFormat>();

        thread::spawn(move || { let _ = receiver; });

        sender
    }
}
