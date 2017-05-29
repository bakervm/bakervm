#[derive(PartialEq, Eq, Hash, Serialize, Deserialize, Debug, Clone)]
pub enum Signal {
    KeyDown,
    KeyUp,
    MouseDown,
    MouseUp,
    Halt,
}
