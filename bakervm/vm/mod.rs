mod image;
mod stack;

use self::image::Image;
use self::stack::Stack;
use definitions::bytecode;
use definitions::typedef::*;
use error::*;
use std::collections::HashMap;
use std::path::Path;

/// The whole state of the VM
pub struct VM {
    image: Image,
    data_stack: Stack,
    ret_addr_stack: Stack,
    inter_reg: HashMap<Word, Address>,
}

impl VM {
    pub fn new() -> VM {
        VM {
            image: Image::default(),
            data_stack: Stack::default(),
            ret_addr_stack: Stack::default(),
            inter_reg: HashMap::new(),
        }
    }

    pub fn exec<P: AsRef<Path>>(&mut self, path: P) -> VMResult<()> {
        self.image.from_path(path)?;

        self.image.check_preamble().chain_err(|| "malformed preamble")?;

        while self.image.pc < self.image.data.len() {
            let byte = self.image.current_byte().chain_err(|| "unable to read current byte")?;

            match byte {
                bytecode::HALT => break,
                bytecode::ADD => {
                    self.data_stack.add().chain_err(|| "unable to execute 'add' instruction")?
                }
                bytecode::SUB => {
                    self.data_stack.sub().chain_err(|| "unable to execute 'sub' instruction")?
                }
                bytecode::MUL => {
                    self.data_stack.mul().chain_err(|| "unable to execute 'mul' instruction")?
                }
                bytecode::DIV => {
                    self.data_stack.div().chain_err(|| "unable to execute 'div' instruction")?
                }
                bytecode::PUSH => {
                    let res: Word = self.image.read_next().chain_err(|| "unable to read word")?;

                    self.data_stack
                        .push_word(res)
                        .chain_err(|| "unable to push value to the data stack")?;
                }
                bytecode::JMP => {
                    let addr: Word = self.image.read_next().chain_err(|| "unable to read word")?;

                    self.image.jmp(addr as Address);
                    continue;
                }
                bytecode::JZ => {
                    let addr: Word = self.image.read_next().chain_err(|| "unable to read word")?;

                    let top_of_stack =
                        self.data_stack
                            .peek_number()
                            .chain_err(|| "unable to get current top of data stack")?;

                    if top_of_stack == 0.0 {
                        self.image.jmp(addr as Address);
                        continue;
                    }
                }
                bytecode::JNZ => {
                    let addr: Word = self.image.read_next().chain_err(|| "unable to read word")?;

                    let top_of_stack =
                        self.data_stack
                            .peek_number()
                            .chain_err(|| "unable to get current top of data stack")?;

                    if top_of_stack != 0.0 {
                        self.image.jmp(addr as Address);
                        continue;
                    }
                }
                bytecode::CALL => {
                    let addr: Address =
                        self.image.read_next().chain_err(|| "unable to read address")?;

                    self.call(addr + 1).chain_err(|| "unable to call function")?;
                    continue;
                }
                bytecode::RET => self.ret().chain_err(|| "unable to return from function call")?,
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

    fn call(&mut self, address: Address) -> VMResult<()> {
        self.ret_addr_stack
            .push_word(address as Word)
            .chain_err(|| "unable to push address to address stack")?;

        // Push return address to clean up the function call frame later
        self.data_stack
            .push_word(address as Word)
            .chain_err(|| "unable to push address to data stack")?;

        self.image.jmp(address);

        Ok(())
    }

    fn ret(&mut self) -> VMResult<()> {
        let return_addr = self.ret_addr_stack
            .pop_word()
            .chain_err(|| "unable to pop address off the address stack")?;

        loop {
            let top_stack_value = self.data_stack.pop_word().chain_err(|| "unable to pop address")?;

            if top_stack_value == return_addr {
                break;
            } else {
                self.data_stack.discard().chain_err(|| "unable to discard value from data stack")?;
            }
        }

        self.image.jmp(return_addr as Address);

        Ok(())
    }
}
