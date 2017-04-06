use definitions::bytecode;
use definitions::typedef::*;
use error::*;
use ieee754::Ieee754;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

struct DisplayRegister {
    width: TinyWord,
    height: TinyWord,
    color_mode: ColorMode,
}

enum ColorMode {
    _1Bit,
    _8Bit,
    _24Bit,
}

pub const BUF_REG_COUNT: usize = 32;

pub struct VM {
    instruction_ptr: Address,
    stack_ptr: Address,
    buf_regs: [Word; BUF_REG_COUNT],
    display_reg: DisplayRegister,
    stack: Vec<Word>,
}

impl VM {
    pub fn new() -> VM {
        VM {
            instruction_ptr: 0,
            stack_ptr: 0,
            buf_regs: [0; BUF_REG_COUNT],
            display_reg: DisplayRegister {
                width: 256,
                height: 512,
                color_mode: ColorMode::_24Bit,
            },
            stack: Vec::new(),
        }
    }

    pub fn exec<P: AsRef<Path>>(&mut self, path: P) -> VMResult<()> {
        let mut image_file = File::open(path).chain_err(|| "unable to open game image")?;
        let mut image_bytes: Vec<Byte> = Vec::new();
        image_file.read_to_end(&mut image_bytes).chain_err(|| "unable to read image")?;

        while self.instruction_ptr < image_bytes.len() {
            let byte = self.current_byte(&image_bytes).chain_err(|| "unable to read current byte")?;

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
                    let res = self.read_word(&image_bytes).chain_err(|| "unable to read word")?;

                    self.push_word(res).chain_err(|| "unable to push value to the stack")?;
                }
                bytecode::JMP => {
                    let addr = self.read_word(&image_bytes).chain_err(|| "unable to read word")?;

                    self.jmp(addr as Address).chain_err(|| "unable to jump")?;
                    continue;
                }
                bytecode::JZ => {
                    let addr = self.read_word(&image_bytes).chain_err(|| "unable to read word")?;

                    let top_of_stack = self.peek_number()
                        .chain_err(|| "unable to get current top of stack")?;

                    if top_of_stack == 0.0 {
                        self.jmp(addr as Address).chain_err(|| "unable to jump")?;
                        continue;
                    }
                }
                bytecode::JNZ => {
                    let addr = self.read_word(&image_bytes).chain_err(|| "unable to read word")?;

                    let top_of_stack = self.peek_number()
                        .chain_err(|| "unable to get current top of stack")?;

                    if top_of_stack != 0.0 {
                        self.jmp(addr as Address).chain_err(|| "unable to jump")?;
                        continue;
                    }
                }
                _ => {
                    bail!("unexpected opcode {:02x} at address {:?}",
                          byte,
                          self.instruction_ptr)
                }
            }

            self.advance_instruction_ptr().chain_err(|| "unable to advance instruction pointer")?;
        }

        Ok(())
    }

    fn push_word(&mut self, value: Word) -> VMResult<()> {
        if !self.stack.is_empty() {
            self.stack_ptr += 1;
        }

        self.stack.push(value);

        Ok(())
    }

    fn push_number(&mut self, value: Number) -> VMResult<()> {
        self.push_word(value.bits())
    }

    fn pop_word(&mut self) -> VMResult<Word> {
        if self.stack.is_empty() {
            bail!("unable to pop word off an empty Stack");
        }

        let res = self.stack.remove(self.stack_ptr);
        if !self.stack.is_empty() {
            self.stack_ptr -= 1;
        }

        Ok(res)
    }

    fn pop_number(&mut self) -> VMResult<Number> {
        let top = self.pop_word().chain_err(|| "unable to pop word off the stack")?;
        Ok(Number::from_bits(top))
    }

    fn add(&mut self) -> VMResult<()> {
        let b = self.pop_number().chain_err(|| "unable to pop word off the stack")?;
        let a = self.pop_number().chain_err(|| "unable to pop word off the stack")?;
        self.push_number(a + b).chain_err(|| "unable to push value to the stack")?;
        Ok(())
    }

    fn sub(&mut self) -> VMResult<()> {
        let b = self.pop_number().chain_err(|| "unable to pop word off the stack")?;
        let a = self.pop_number().chain_err(|| "unable to pop word off the stack")?;
        self.push_number(a - b).chain_err(|| "unable to push to stack")?;
        Ok(())
    }


    fn mul(&mut self) -> VMResult<()> {
        let b = self.pop_number().chain_err(|| "unable to pop word off the stack")?;
        let a = self.pop_number().chain_err(|| "unable to pop word off the stack")?;
        self.push_number(a * b).chain_err(|| "unable to push to stack")?;
        Ok(())
    }


    fn div(&mut self) -> VMResult<()> {
        let b = self.pop_number().chain_err(|| "unable to pop word off the stack")?;
        let a = self.pop_number().chain_err(|| "unable to pop word off the stack")?;
        self.push_number(a / b).chain_err(|| "unable to push to stack")?;
        Ok(())
    }

    fn jmp(&mut self, addr: Address) -> VMResult<()> {
        self.instruction_ptr = addr;
        Ok(())
    }

    fn read_word(&mut self, bytes: &Vec<Byte>) -> VMResult<Word> {
        // Build a Word from single bytes
        let mut res: Word = 0;
        for _ in 0..8 {
            res <<= 8;
            self.advance_instruction_ptr().chain_err(|| "unable to advance instruction pointer")?;
            let current_byte = self.current_byte(&bytes)
                .chain_err(|| "unable to read current byte")?;
            res |= current_byte as Word;
        }

        Ok(res)
    }


    fn read_small_word(&mut self, bytes: &Vec<Byte>) -> VMResult<SmallWord> {
        // Build a Word from single bytes
        let mut res: SmallWord = 0;
        for _ in 0..4 {
            res <<= 8;
            self.advance_instruction_ptr().chain_err(|| "unable to advance instruction pointer")?;
            let current_byte = self.current_byte(&bytes)
                .chain_err(|| "unable to read current byte")?;
            res |= current_byte as SmallWord;
        }

        Ok(res)
    }


    fn read_tiny_word(&mut self, bytes: &Vec<Byte>) -> VMResult<TinyWord> {
        // Build a Word from single bytes
        let mut res: TinyWord = 0;
        for _ in 0..2 {
            res <<= 8;
            self.advance_instruction_ptr().chain_err(|| "unable to advance instruction pointer")?;
            let current_byte = self.current_byte(&bytes)
                .chain_err(|| "unable to read current byte")?;
            res |= current_byte as TinyWord;
        }

        Ok(res)
    }



    fn advance_instruction_ptr(&mut self) -> VMResult<()> {
        self.instruction_ptr += 1;
        Ok(())
    }

    fn current_byte(&mut self, bytes: &Vec<Byte>) -> VMResult<Byte> {
        if self.instruction_ptr < bytes.len() {
            Ok(bytes[self.instruction_ptr])
        } else {
            bail!("instruction pointer out of bounds");
        }
    }

    fn peek_word(&mut self) -> VMResult<Word> {
        if self.stack_ptr < self.stack.len() {
            Ok(self.stack[self.stack_ptr])
        } else {
            bail!("stack pointer out of bounds");
        }
    }

    fn peek_number(&mut self) -> VMResult<Number> {
        let top = self.peek_word().chain_err(|| "unable to peek for word")?;
        Ok(Number::from_bits(top))
    }

    fn print(&mut self) -> VMResult<()> {
        let top = self.peek_number().chain_err(|| "unable to get current top of stack")?;
        println!("{:?}", top);
        Ok(())
    }
}
