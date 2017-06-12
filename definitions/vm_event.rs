//! Interrupts for communicating with the VM from the outside and also for
//! letting the VM communicate with the outside

use std::str::FromStr;
use typedef::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum VMEvent {
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

impl VMEvent {
    pub fn get_type(&self) -> VMEventType {
        match self {
            &VMEvent::KeyDown(..) => VMEventType::KeyDown,
            &VMEvent::KeyUp(..) => VMEventType::KeyUp,
            &VMEvent::MouseDown { .. } => VMEventType::MouseDown,
            &VMEvent::MouseUp { .. } => VMEventType::MouseUp,
            &VMEvent::Halt => VMEventType::Halt,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
pub enum VMEventType {
    KeyDown,
    KeyUp,
    MouseDown,
    MouseUp,
    Halt,
}

impl FromStr for VMEventType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "%key_down%" => Ok(VMEventType::KeyDown),
            "%key_up%" => Ok(VMEventType::KeyUp),
            "%mouse_down%" => Ok(VMEventType::MouseDown),
            "%mouse_up%" => Ok(VMEventType::MouseUp),
            "%halt%" => Ok(VMEventType::Halt),
            _ => Err("unable to parse event"),
        }
    }
}
