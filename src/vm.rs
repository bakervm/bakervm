use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use error::*;
use bytecode;

pub struct VM {
    ip: usize,
    sp: usize,
    globals: Vec<u64>,
    stack: Vec<u64>,
}

impl VM {
    pub fn new(startip: usize) -> VM {
        VM {
            ip: startip,
            sp: 0,
            globals: Vec::new(),
            stack: Vec::new(),
        }
    }

    pub fn exec<P: AsRef<Path>>(&mut self, path: P) -> ExecResult<()> {
        let mut image_file = File::open(path).chain_err(|| "unable to open game image")?;
        let mut image_string = String::new();
        image_file.read_to_string(&mut image_string).chain_err(|| "unable to read image")?;

        let mut byte_iter = image_string.bytes();
        let mut outer_byte = byte_iter.next();
        while let Some(byte) = outer_byte {
            match byte {
                bytecode::HALT => break,
                bytecode::ADD => self.add(),
                bytecode::SUB => self.sub(),
                bytecode::PRINT => self.print(),
                bytecode::PUSH => {
                    // Build a u32 from single bytes
                    let mut res: u32 = 0;
                    for _ in 0..4 {
                        res <<= 8;
                        res |= byte_iter.next().unwrap() as u32;
                    }

                    self.push(res);
                }
                _ => panic!("Unexpected opcode: {:08x}", byte),
            }

            outer_byte = byte_iter.next();
        }

        Ok(())
    }

    fn push(&mut self, value: u32) {
        if !self.stack.is_empty() {
            self.sp += 1;
        }

        self.stack.push(value as u64);
    }

    fn pop(&mut self) -> u64 {
        assert!(!self.stack.is_empty(),
                "Unable to pop value off an empty Stack");
        let res = self.stack.remove(self.sp);
        if !self.stack.is_empty() {
            self.sp -= 1;
        }
        res
    }

    fn add(&mut self) {
        let b = self.pop();
        let a = self.pop();
        self.push((a + b) as u32);
    }

    fn sub(&mut self) {
        let b = self.pop();
        let a = self.pop();
        self.push((a - b) as u32);
    }

    fn print(&mut self) {
        println!("{:?}", self.stack[self.sp]);
    }
}
