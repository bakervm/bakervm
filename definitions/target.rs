use typedef::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Target {
    Framebuffer(Address),
    ValueIndex(Address),
    InputRegister,
    Stack,
}
