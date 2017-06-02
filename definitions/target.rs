use typedef::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Target {
    Framebuffer,
    ValueIndex(Address),
    Stack,
}
