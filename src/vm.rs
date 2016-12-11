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
    pub fn new() -> VM {
        VM {
            ip: 0,
            sp: 0,
            globals: Vec::new(),
            stack: Vec::new(),
        }
    }

    pub fn exec<P: AsRef<Path>>(&mut self, path: P) -> VMResult<()> {
        let mut image_file = File::open(path).chain_err(|| "unable to open game image")?;
        let mut image_string = String::new();
        image_file.read_to_string(&mut image_string).chain_err(|| "unable to read image")?;

        let mut byte_iter = image_string.bytes();
        let mut outer_byte = byte_iter.next();
        while let Some(byte) = outer_byte {
            match byte {
                bytecode::HALT => break,
                bytecode::ADD => self.add().chain_err(|| "unable to execute 'add' instruction")?,
                bytecode::SUB => self.sub().chain_err(|| "unable to execute 'sub' instruction")?,
                bytecode::MUL => self.mul().chain_err(|| "unable to execute 'mul' instruction")?,
                bytecode::DIV => self.div().chain_err(|| "unable to execute 'div' instruction")?,
                bytecode::PRINT => {
                    self.print().chain_err(|| "unable to execute 'print' instruction")?
                }
                bytecode::PUSH => {
                    // Build a u32 from single bytes
                    let mut res: u32 = 0;
                    for _ in 0..4 {
                        res <<= 8;
                        res |= byte_iter.next().unwrap() as u32;
                    }

                    self.push(res).chain_err(|| "unable to push value to the stack")?;
                }
                _ => bail!("unexpected opcode: {:08x}", byte),
            }

            outer_byte = byte_iter.next();
        }

        Ok(())
    }

    fn push(&mut self, value: u32) -> VMResult<()> {
        if !self.stack.is_empty() {
            self.sp += 1;
        }

        self.stack.push(value as u64);

        Ok(())
    }

    fn pop(&mut self) -> VMResult<u64> {
        if self.stack.is_empty() {
            bail!("unable to pop value off an empty Stack");
        }

        let res = self.stack.remove(self.sp);
        if !self.stack.is_empty() {
            self.sp -= 1;
        }

        Ok(res)
    }

    fn add(&mut self) -> VMResult<()> {
        let b = self.pop().chain_err(|| "unable to pop value off the stack")?;
        let a = self.pop().chain_err(|| "unable to pop value off the stack")?;
        self.push((a + b) as u32).chain_err(|| "unable to push value to the stack")?;
        Ok(())
    }

    fn sub(&mut self) -> VMResult<()> {
        let b = self.pop().chain_err(|| "unable to pop value off the stack")?;
        let a = self.pop().chain_err(|| "unable to pop value off the stack")?;
        self.push((a - b) as u32).chain_err(|| "unable to push to stack")?;
        Ok(())
    }


    fn mul(&mut self) -> VMResult<()> {
        let b = self.pop().chain_err(|| "unable to pop value off the stack")?;
        let a = self.pop().chain_err(|| "unable to pop value off the stack")?;
        self.push((a * b) as u32).chain_err(|| "unable to push to stack")?;
        Ok(())
    }


    fn div(&mut self) -> VMResult<()> {
        let b = self.pop().chain_err(|| "unable to pop value off the stack")?;
        let a = self.pop().chain_err(|| "unable to pop value off the stack")?;
        self.push((a / b) as u32).chain_err(|| "unable to push to stack")?;
        Ok(())
    }



    fn print(&mut self) -> VMResult<()> {
        println!("{:?}", self.stack[self.sp]);
        Ok(())
    }
}
