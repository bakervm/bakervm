//! Interrupts for communicating with the VM from the outside and also for
//! letting the VM communicate with the outside

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
    MouseMove { x: Address, y: Address },
    Halt,
}
