use signal::Signal;
use value::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExternalInterrupt {
    pub signal: Signal,
    pub args: Vec<Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum InternalInterrupt {
    FlushFramebuffer,
}
