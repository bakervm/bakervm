use definitions::typedef::*;

#[derive(Debug)]
pub struct Interrupt {
    pub signal_id: u64,
    pub arguments: Vec<Word>,
}
