use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use error::*;
use bytecode;

pub struct VM {
    instruction_ptr: usize,
    stack_ptr: usize,
    globals: Vec<u64>,
    stack: Vec<u64>,
}

impl VM {
    pub fn new() -> VM {
        VM {
            instruction_ptr: 0,
            stack_ptr: 0,
            globals: Vec::new(),
            stack: Vec::new(),
        }
    }

    pub fn exec<P: AsRef<Path>>(&mut self, path: P) -> VMResult<()> {
        let mut image_file = File::open(path).chain_err(|| "unable to open game image")?;
        let mut image_string = String::new();
        image_file.read_to_string(&mut image_string).chain_err(|| "unable to read image")?;

        let byte_iter: Vec<u8> = image_string.bytes().collect();
        while self.instruction_ptr < byte_iter.len() {
            let byte = self.current_byte(&byte_iter)
                .chain_err(|| "unable to read current byte")?;

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
                    let res = self.read_word(&byte_iter).chain_err(|| "unable to read word")?;

                    self.push(res).chain_err(|| "unable to push value to the stack")?;
                }
                bytecode::JMP => {
                    let res = self.read_word(&byte_iter).chain_err(|| "unable to read word")?;

                    self.jmp(res).chain_err(|| "unable to jump")?;
                    continue;
                }
                bytecode::JZ => {
                    let res = self.read_word(&byte_iter).chain_err(|| "unable to read word")?;

                    let top_of_stack = self.peek()
                        .chain_err(|| "unable to get current top of stack")?;

                    if top_of_stack == 0 {
                        self.jmp(res).chain_err(|| "unable to jump")?;
                        continue;
                    }
                }
                bytecode::JNZ => {
                    let res = self.read_word(&byte_iter).chain_err(|| "unable to read word")?;

                    let top_of_stack = self.peek()
                        .chain_err(|| "unable to get current top of stack")?;

                    if top_of_stack != 0 {
                        self.jmp(res).chain_err(|| "unable to jump")?;
                        continue;
                    }
                }
                _ => {
                    bail!("unexpected opcode {:02x} at address {:08x}",
                          byte,
                          self.instruction_ptr)
                }
            }

            self.advance_instruction_ptr().chain_err(|| "unable to advance instruction pointer")?;
        }

        Ok(())
    }

    fn push(&mut self, value: u32) -> VMResult<()> {
        if !self.stack.is_empty() {
            self.stack_ptr += 1;
        }

        self.stack.push(value as u64);

        Ok(())
    }

    fn pop(&mut self) -> VMResult<u64> {
        if self.stack.is_empty() {
            bail!("unable to pop value off an empty Stack");
        }

        let res = self.stack.remove(self.stack_ptr);
        if !self.stack.is_empty() {
            self.stack_ptr -= 1;
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

    fn jmp(&mut self, addr: u32) -> VMResult<()> {
        self.instruction_ptr = addr as usize;
        Ok(())
    }

    fn read_word(&mut self, bytes: &Vec<u8>) -> VMResult<u32> {
        // Build a u32 from single bytes
        let mut res: u32 = 0;
        for _ in 0..4 {
            res <<= 8;
            self.advance_instruction_ptr().chain_err(|| "unable to advance instruction pointer")?;
            let current_byte = self.current_byte(&bytes)
                .chain_err(|| "unable to read current byte")?;
            res |= current_byte as u32;
        }

        Ok(res)
    }

    fn advance_instruction_ptr(&mut self) -> VMResult<()> {
        self.instruction_ptr += 1;
        Ok(())
    }

    fn current_byte(&mut self, bytes: &Vec<u8>) -> VMResult<u8> {
        if self.instruction_ptr < bytes.len() {
            Ok(bytes[self.instruction_ptr])
        } else {
            bail!("instruction pointer out of bounds");
        }
    }

    fn peek(&mut self) -> VMResult<u64> {
        if self.stack_ptr < self.stack.len() {
            Ok(self.stack[self.stack_ptr])
        } else {
            bail!("stack pointer out of bounds");
        }
    }

    fn print(&mut self) -> VMResult<()> {
        let top = self.peek().chain_err(|| "unable to get current top of stack")?;
        println!("{:?}", top);
        Ok(())
    }
}
