use definitions::typedef::*;
use error::*;
use ieee754::Ieee754;

#[derive(Default, Debug)]
pub struct Stack {
    /// The stack pointer
    pub ptr: Address,
    pub data: Vec<Word>,
}

impl Stack {
    pub fn truncate(&mut self, len: usize) {
        self.data.truncate(len);

        self.recalculate_ptr();
    }

    pub fn append(&mut self, other: &mut Vec<Word>) {
        self.data.append(other);

        self.recalculate_ptr();
    }

    pub fn recalculate_ptr(&mut self) {
        self.ptr = if self.data.is_empty() {
            0
        } else {
            self.data.len() - 1
        };
    }

    pub fn peek_word(&mut self) -> VMResult<Word> {
        if self.ptr < self.data.len() {
            Ok(self.data[self.ptr])
        } else {
            bail!("stack pointer out of bounds");
        }
    }

    pub fn peek_number(&mut self) -> VMResult<Number> {
        let top = self.peek_word().chain_err(|| "unable to peek for word")?;
        Ok(Number::from_bits(top))
    }

    pub fn push_word(&mut self, value: Word) -> VMResult<()> {
        self.data.push(value);

        self.recalculate_ptr();

        Ok(())
    }

    pub fn push_number(&mut self, value: Number) -> VMResult<()> {
        self.push_word(value.bits())
    }

    pub fn pop_word(&mut self) -> VMResult<Word> {
        if self.data.is_empty() {
            bail!("unable to pop word off an empty Stack");
        }

        let res = self.data.remove(self.ptr);

        self.recalculate_ptr();

        Ok(res)
    }

    pub fn pop_number(&mut self) -> VMResult<Number> {
        let top = self.pop_word().chain_err(|| "unable to pop word off the stack")?;
        Ok(Number::from_bits(top))
    }

    pub fn add(&mut self) -> VMResult<()> {
        let b = self.pop_number().chain_err(|| "unable to pop word off the stack")?;
        let a = self.pop_number().chain_err(|| "unable to pop word off the stack")?;
        self.push_number(a + b).chain_err(|| "unable to push value to the stack")?;
        Ok(())
    }

    pub fn sub(&mut self) -> VMResult<()> {
        let b = self.pop_number().chain_err(|| "unable to pop word off the stack")?;
        let a = self.pop_number().chain_err(|| "unable to pop word off the stack")?;
        self.push_number(a - b).chain_err(|| "unable to push to stack")?;
        Ok(())
    }


    pub fn mul(&mut self) -> VMResult<()> {
        let b = self.pop_number().chain_err(|| "unable to pop word off the stack")?;
        let a = self.pop_number().chain_err(|| "unable to pop word off the stack")?;
        self.push_number(a * b).chain_err(|| "unable to push to stack")?;
        Ok(())
    }


    pub fn div(&mut self) -> VMResult<()> {
        let b = self.pop_number().chain_err(|| "unable to pop word off the stack")?;
        let a = self.pop_number().chain_err(|| "unable to pop word off the stack")?;
        self.push_number(a / b).chain_err(|| "unable to push to stack")?;
        Ok(())
    }
}
