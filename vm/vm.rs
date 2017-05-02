use definitions::bytecode;
use definitions::typedef::*;
use error::*;
use ieee754::Ieee754;
use num::traits::Num;
use num::traits::cast::FromPrimitive;
use output::Mountable;
use std::fs::File;
use std::io::prelude::*;
use std::mem;
use std::ops::{BitOrAssign, ShlAssign};
use std::path::Path;

/// We can adjust the buffer register count here
pub const BUF_REG_COUNT: usize = 32;

/// A register type for comparing two values
struct CompareRegister {
    cra: Word,
    crb: Word,
}

#[derive(Default)]
struct ImageData {
    /// The currently loaded image
    data: Image,
    /// The program counter
    pc: Address,
}

impl ImageData {
    pub fn from_path<P: AsRef<Path>>(&mut self, path: P) -> VMResult<()> {
        let mut image_file = File::open(path).chain_err(|| "unable to open game image file")?;
        image_file.read_to_end(&mut self.data).chain_err(|| "unable to read game image file")?;

        Ok(())
    }

    fn advance_pc(&mut self) {
        self.pc += 1;
    }

    fn current_byte(&mut self) -> VMResult<Byte> {
        if self.pc < self.data.len() {
            Ok(self.data[self.pc])
        } else {
            bail!("program counter out of bounds");
        }
    }

    fn jmp(&mut self, addr: Address) {
        self.pc = addr;
    }

    fn read<T: FromPrimitive + Num + ShlAssign<u8> + BitOrAssign>(&mut self) -> VMResult<T> {
        // Build a Word from single bytes
        let mut res: T = T::zero();

        let length = mem::size_of::<T>();

        for _ in 0..length {
            res <<= 8u8;
            self.advance_pc();
            let current_byte = self.current_byte().chain_err(|| "unable to read current byte")?;
            if let Some(number) = T::from_u8(current_byte) {
                res |= number;
            } else {
                bail!("unable to convert from u8");
            }

        }

        Ok(res)
    }
}

#[derive(Default)]
struct StackData {
    /// The stack pointer
    ptr: Address,
    data: Vec<Word>,
}

/// The whole state of the VM
pub struct VM {
    /// The buffer registers
    buf_regs: [Word; BUF_REG_COUNT],
    cmp_reg: CompareRegister,
    image: ImageData,
    stack: StackData,
}

impl VM {
    pub fn new() -> VM {
        VM {
            image: ImageData::default(),
            stack: StackData::default(),
            buf_regs: [0; BUF_REG_COUNT],
            cmp_reg: CompareRegister { cra: 0, crb: 0 },
        }
    }

    pub fn exec<P: AsRef<Path>>(&mut self, path: P) -> VMResult<()> {
        self.image.from_path(path)?;

        while self.image.pc < self.image.data.len() {
            let byte = self.image.current_byte().chain_err(|| "unable to read current byte")?;

            match byte {
                bytecode::HALT => break,
                bytecode::ADD => self.add().chain_err(|| "unable to execute 'add' instruction")?,
                bytecode::SUB => self.sub().chain_err(|| "unable to execute 'sub' instruction")?,
                bytecode::MUL => self.mul().chain_err(|| "unable to execute 'mul' instruction")?,
                bytecode::DIV => self.div().chain_err(|| "unable to execute 'div' instruction")?,
                bytecode::PUSH => {
                    let res = self.image.read().chain_err(|| "unable to read word")?;

                    self.push_word(res).chain_err(|| "unable to push value to the stack")?;
                }
                bytecode::JMP => {
                    let addr: Word = self.image.read().chain_err(|| "unable to read word")?;

                    self.image.jmp(addr as Address);
                    continue;
                }
                bytecode::JZ => {
                    let addr: Word = self.image.read().chain_err(|| "unable to read word")?;

                    let top_of_stack = self.peek_number()
                        .chain_err(|| "unable to get current top of stack")?;

                    if top_of_stack == 0.0 {
                        self.image.jmp(addr as Address);
                        continue;
                    }
                }
                bytecode::JNZ => {
                    let addr: Word = self.image.read().chain_err(|| "unable to read word")?;

                    let top_of_stack = self.peek_number()
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

    fn push_word(&mut self, value: Word) -> VMResult<()> {
        if !self.stack.data.is_empty() {
            self.stack.ptr += 1;
        }

        self.stack.data.push(value);

        Ok(())
    }

    fn push_number(&mut self, value: Number) -> VMResult<()> {
        self.push_word(value.bits())
    }

    fn pop_word(&mut self) -> VMResult<Word> {
        if self.stack.data.is_empty() {
            bail!("unable to pop word off an empty Stack");
        }

        let res = self.stack.data.remove(self.stack.ptr);
        if !self.stack.data.is_empty() {
            self.stack.ptr -= 1;
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



    fn peek_word(&mut self) -> VMResult<Word> {
        if self.stack.ptr < self.stack.data.len() {
            Ok(self.stack.data[self.stack.ptr])
        } else {
            bail!("stack pointer out of bounds");
        }
    }

    fn peek_number(&mut self) -> VMResult<Number> {
        let top = self.peek_word().chain_err(|| "unable to peek for word")?;
        Ok(Number::from_bits(top))
    }
}
