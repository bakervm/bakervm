//! Interrupts for communicating with the VM from the outside and also for
//! letting the VM communicate with the outside

use std::str::FromStr;
use typedef::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ExternalInterrupt {
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum InternalInterrupt {
    FlushFramebuffer,
}

impl FromStr for InternalInterrupt {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(InternalInterrupt::FlushFramebuffer),
            _ => Err("unable to parse interrupt"),
        }
    }
}
