use definitions::bytecode;
use definitions::typedef::*;
use error::*;
use num::traits::Num;
use num::traits::cast::FromPrimitive;
use std::fs::File;
use std::io::prelude::*;
use std::mem;
use std::ops::{BitOrAssign, ShlAssign};
use std::path::Path;

#[derive(Default, Debug)]
pub struct Image {
    /// The currently loaded image
    pub data: ImageData,
    /// The program counter
    pub pc: Address,
}

impl Image {
    pub fn load<P: AsRef<Path>>(&mut self, path: P) -> VMResult<()> {
        let mut image_file = File::open(path).chain_err(|| "unable to open game image file")?;
        image_file.read_to_end(&mut self.data).chain_err(|| "unable to read game image file")?;

        Ok(())
    }

    pub fn check_preamble(&mut self) -> VMResult<()> {
        let preamble = String::from(bytecode::PREAMBLE);

        let mut char_vec: Vec<Byte> = Vec::new();
        char_vec.push(self.current_byte().chain_err(|| "unable to read byte")?);

        for _ in 1..preamble.len() {
            char_vec.push(self.read_next().chain_err(|| "unable to read byte")?);
        }

        let magic_word = String::from_utf8(char_vec).chain_err(|| "invalid UTF-8 character")?;

        ensure!(
            magic_word == bytecode::PREAMBLE,
            "unable to find magic word"
        );

        self.advance_pc();

        Ok(())
    }

    pub fn advance_pc(&mut self) {
        self.pc += 1;
    }

    pub fn current_byte(&mut self) -> VMResult<Byte> {
        if self.pc < self.data.len() {
            Ok(self.data[self.pc])
        } else {
            bail!("program counter out of bounds");
        }
    }

    pub fn jmp(&mut self, addr: Address) {
        self.pc = addr;
    }

    pub fn read_next<T: FromPrimitive + Num + ShlAssign<u8> + BitOrAssign>(&mut self)
        -> VMResult<T> {
        // Build a Word from single bytes
        let mut res: T = T::zero();

        let length = mem::size_of::<T>();

        for i in 0..length {
            if i > 0 {
                res <<= 8u8;
            }
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
