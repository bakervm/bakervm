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
    call_stack: Stack,
    gc_stack: Stack,
    yield_stack: Stack,
    yield_count_stack: Stack,
    inter_reg: HashMap<Word, Address>,
}

impl VM {
    pub fn new() -> VM {
        VM {
            image: Image::default(),
            data_stack: Stack::default(),
            call_stack: Stack::default(),
            gc_stack: Stack::default(),
            yield_stack: Stack::default(),
            yield_count_stack: Stack::default(),
            inter_reg: HashMap::new(),
        }
    }

    pub fn exec<P: AsRef<Path>>(&mut self, path: P) -> VMResult<()> {
        self.image.load(path).chain_err(|| "unable to load image")?;

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

    fn call(&mut self, address: Address) -> VMResult<()> {
        self.call_stack
            .push_word(address as Word)
            .chain_err(|| "unable to push address to address stack")?;

        let stack_ptr = self.data_stack.ptr;

        self.gc_stack
            .push_word((stack_ptr + 1) as Word)
            .chain_err(|| "unable to push activation frame to GC stack")?;

        self.yield_count_stack.push_word(0)?;

        self.image.jmp(address);

        Ok(())
    }

    fn ret(&mut self) -> VMResult<()> {
        let return_addr =
            self.call_stack.pop_word().chain_err(|| "unable to pop address off the address stack")?;

        let activation_frame =
            self.gc_stack.pop_word().chain_err(|| "unable to pop activation frame from GC stack")?;

        self.data_stack
            .truncate(activation_frame as usize)
            .chain_err(|| "unable to truncate data stack")?;

        let yield_count = self.yield_count_stack.pop_word()?;

        for _ in 0..yield_count {
            let value = self.yield_stack.pop_word()?;

            self.data_stack.push_word(value)?;
        }

        self.image.jmp(return_addr as Address);

        Ok(())
    }

    fn yld(&mut self) -> VMResult<()> {
        if self.call_stack.data.is_empty() {
            bail!("unable to yield values from an empty call stack");
        }

        let value =
            self.data_stack.pop_word().chain_err(|| "unable to pop value off the data stack")?;

        self.yield_stack.push_word(value).chain_err(|| "unable to push value to the yield stack")?;

        // increase yield count
        let yield_count = self.yield_count_stack
            .pop_word()
            .chain_err(|| "unable to pop value off the data stack")?;

        self.yield_count_stack
            .push_word(yield_count + 1)
            .chain_err(|| "unable to increase yield count")?;

        Ok(())
    }
}
