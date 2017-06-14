//! Interrupts for communicating with the VM from the outside and also for
//! letting the VM communicate with the outside

use std::str::FromStr;
use typedef::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Event {
    KeyDown(Address),
    KeyUp(Address),
    MouseDown {
        button: Address,
        x: Address,
        y: Address,
    },
    MouseUp {
        button: Address,
        x: Address,
        y: Address,
    },
    Halt,
}

impl FromStr for Event {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Event::Halt),
            _ => Err("unable to parse interrupt"),
        }
    }
}
