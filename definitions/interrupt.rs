//! Interrupts for communicating with the VM from the outside and also for
//! letting the VM communicate with the outside

use typedef::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ExternalInterrupt {
    KeyDown(Integer),
    KeyUp,
    MouseDown(Integer),
    MouseUp(Integer),
    Halt,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum InternalInterrupt {
    FlushFramebuffer,
}
