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
