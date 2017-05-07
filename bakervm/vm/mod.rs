mod image;
mod stack;
mod call_stack;

use self::call_stack::Call;
use self::call_stack::CallStack;
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
    call_stack: CallStack,
    inter_reg: HashMap<Word, Address>,
}

impl VM {
    pub fn new() -> VM {
        VM {
            image: Image::default(),
            data_stack: Stack::default(),
            call_stack: CallStack::default(),
            inter_reg: HashMap::new(),
        }
    }

    pub fn exec<P: AsRef<Path>>(&mut self, path: P) -> VMResult<()> {
        self.image.load(path).chain_err(|| "unable to load image")?;

        self.image.check_preamble().chain_err(|| "malformed preamble")?;

        while self.image.pc < self.image.data.len() {
            self.handle_intertupts().chain_err(|| "unable to handle interrupts")?;

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
                bytecode::YLD => self.yld().chain_err(|| "unable to yield value")?,
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

    fn handle_intertupts(&mut self) -> VMResult<()> {
        if self.inter_reg.is_empty() {
            Ok(())
        } else {
            unimplemented!();
        }
    }

    fn call(&mut self, address: Address) -> VMResult<()> {
        let stack_len = self.data_stack.data.len();

        let call = Call::new(address, stack_len);

        self.call_stack.push(call);

        self.image.jmp(address);

        Ok(())
    }

    fn ret(&mut self) -> VMResult<()> {
        if self.call_stack.is_empty() {
            bail!("unable to yield values from an empty call stack");
        }

        // We unwrap here because we already checked if the call stack is empty
        let mut call = self.call_stack.pop().unwrap();

        self.data_stack.truncate(call.gc).chain_err(|| "unable to truncate data stack")?;

        self.data_stack.data.append(&mut call.yield_stack);

        self.image.jmp(call.ret_addr);

        Ok(())
    }

    fn yld(&mut self) -> VMResult<()> {
        if self.call_stack.is_empty() {
            bail!("unable to yield values from an empty call stack");
        }

        // We unwrap here because we already checked if the call stack is empty
        let mut call = self.call_stack.pop().unwrap();

        let value =
            self.data_stack.pop_word().chain_err(|| "unable to pop value off the data stack")?;

        call.yield_stack.push(value);

        self.call_stack.push(call);

        Ok(())
    }
}
