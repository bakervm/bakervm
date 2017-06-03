//! A Target is an abstract representation of a memory section inside the VM

use typedef::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Target {
    Framebuffer,
    ValueIndex(Address),
    Stack,
}
