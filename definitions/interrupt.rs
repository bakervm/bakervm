use value::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExternalInterrupt {
    pub signal_id: usize,
    pub args: Vec<Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum InternalInterrupt {
    FlushFramebuffer,
}
