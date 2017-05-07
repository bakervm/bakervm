use definitions::typedef::*;

pub struct Call {
    pub yield_stack: Vec<Word>,
    pub gc: usize,
    pub ret_addr: Address,
}

impl Call {
    pub fn new(ret_addr: Address, gc: usize) -> Call {
        Call {
            yield_stack: Vec::new(),
            gc: gc,
            ret_addr: ret_addr,
        }
    }
}

pub type CallStack = Vec<Call>;
