use definitions::bytecode;
use definitions::typedef::*;
use error::*;
use image::Image;
use output::Mountable;
use stack::Stack;
use std::path::Path;


/// The whole state of the VM
pub struct VM {
    image: Image,
    stack: Stack,
}

impl VM {
    pub fn new() -> VM {
        VM {
            image: Image::default(),
            stack: Stack::default(),
        }
    }

    pub fn exec<P: AsRef<Path>>(&mut self, path: P) -> VMResult<()> {
        self.image.from_path(path)?;

        while self.image.pc < self.image.data.len() {
            let byte = self.image.current_byte().chain_err(|| "unable to read current byte")?;

            match byte {
                bytecode::HALT => break,
                bytecode::ADD => {
                    self.stack.add().chain_err(|| "unable to execute 'add' instruction")?
                }
                bytecode::SUB => {
                    self.stack.sub().chain_err(|| "unable to execute 'sub' instruction")?
                }
                bytecode::MUL => {
                    self.stack.mul().chain_err(|| "unable to execute 'mul' instruction")?
                }
                bytecode::DIV => {
                    self.stack.div().chain_err(|| "unable to execute 'div' instruction")?
                }
                bytecode::PUSH => {
                    let res = self.image.read_next().chain_err(|| "unable to read word")?;

                    self.stack.push_word(res).chain_err(|| "unable to push value to the stack")?;
                }
                bytecode::JMP => {
                    let addr: Word = self.image.read_next().chain_err(|| "unable to read word")?;

                    self.image.jmp(addr as Address);
                    continue;
                }
                bytecode::JZ => {
                    let addr: Word = self.image.read_next().chain_err(|| "unable to read word")?;

                    let top_of_stack = self.stack
                        .peek_number()
                        .chain_err(|| "unable to get current top of stack")?;

                    if top_of_stack == 0.0 {
                        self.image.jmp(addr as Address);
                        continue;
                    }
                }
                bytecode::JNZ => {
                    let addr: Word = self.image.read_next().chain_err(|| "unable to read word")?;

                    let top_of_stack = self.stack
                        .peek_number()
                        .chain_err(|| "unable to get current top of stack")?;

                    if top_of_stack != 0.0 {
                        self.image.jmp(addr as Address);
                        continue;
                    }
                }
                _ => {
                    bail!(
                        "unexpected opcode {:02x} at address {:?}",
                        byte,
                        self.image.pc
                    )
                }
            }

            self.image.advance_pc();
        }

        Ok(())
    }

    pub fn mount<T: Mountable>(&mut self, device: T) -> VMResult<()> {
        device.run();

        Ok(())
    }
}
