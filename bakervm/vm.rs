use definitions::program::{Instruction, PREAMBLE, Program, Target, Value};
use definitions::typedef::*;
use error::*;
use std::cmp::Ordering;
use std::collections::LinkedList;

const STACK_COUNT: usize = 2;
const REGISTER_COUNT: usize = 4;

/// The whole state of the VM
pub struct VM {
    /// The instructions that are currently executed
    image_data: Vec<Instruction>,
    /// The current program counter
    pc: Address,
    data_stacks: [LinkedList<Value>; STACK_COUNT],
    data_registers: [Value; REGISTER_COUNT],
    /// A register for holding infomation about a recent comparison
    cmp_register: Option<Ordering>,
    call_stack: LinkedList<Address>,
    /// A boolean used for lock the program counter
    pc_locked: bool,
}

impl VM {
    /// Creates a new VM state for executing programs
    pub fn new() -> VM {
        VM {
            image_data: Vec::new(),
            pc: 0,
            data_stacks: [LinkedList::new(), LinkedList::new()],
            data_registers: [Value::Nil, Value::Nil, Value::Nil, Value::Nil],
            cmp_register: None,
            call_stack: LinkedList::new(),
            pc_locked: false,
        }
    }

    // # Maintainance functions

    /// Executes the given program
    pub fn exec(&mut self, program: Program) -> VMResult<()> {
        self.reset();
        self.load_program(program)?;

        while self.pc < self.image_data.len() {
            let current_instruction = self.image_data[self.pc].clone();

            match current_instruction {
                Instruction::Halt => break,

                Instruction::Add(dest, src) => self.add(dest, src)?,
                Instruction::Sub(dest, src) => self.sub(dest, src)?,
                Instruction::Div(dest, src) => self.div(dest, src)?,
                Instruction::Mul(dest, src) => self.mul(dest, src)?,
                Instruction::Mod(dest, src) => self.modulo(dest, src)?,

                Instruction::Cmp(target_a, target_b) => self.cmp(target_a, target_b)?,
                Instruction::Jmp(addr) => self.jmp(addr),
                Instruction::JmpLt(addr) => self.jmp_lt(addr),
                Instruction::JmpGt(addr) => self.jmp_gt(addr),
                Instruction::JmpEq(addr) => self.jmp_eq(addr),

                Instruction::Push(dest, value) => self.push(dest, value),
                Instruction::Mov(dest, src) => self.mov(dest, src)?,
                Instruction::Swp(target_a, target_b) => self.swp(target_a, target_b)?,

                Instruction::Call(addr) => self.call(addr),
                Instruction::Ret => self.ret()?,
            }

            self.advance_pc();
        }

        Ok(())
    }

    /// Loads the instructions of the given program to the VM's state
    fn load_program(&mut self, program: Program) -> VMResult<()> {
        if program.preamble != String::from(PREAMBLE) {
            bail!("invalid preamble");
        } else if program.version != String::from(env!("CARGO_PKG_VERSION")) {
            bail!("invalid version");
        } else {
            self.image_data = program.instructions;
            Ok(())
        }
    }

    /// Resets the VM to a clean state
    fn reset(&mut self) {
        *self = VM::new();
    }

    fn reset_cmp(&mut self) {
        self.cmp_register = None;
    }

    /// Advances the program counter
    fn advance_pc(&mut self) {
        if self.pc_locked {
            self.pc_locked = false;
        } else {
            self.pc += 1;
        }
    }

    /// Return the value at the specified target. The value will be consumed
    /// from the target.
    fn pop(&mut self, target: &Target) -> VMResult<Value> {
        match target {
            &Target::Register(index) => {
                let value = self.data_registers[index].clone();
                self.data_registers[index] = Value::Nil;

                Ok(value)
            }
            &Target::Stack(index) => {
                if let Some(value) = self.data_stacks[index].pop_front() {
                    Ok(value)
                } else {
                    bail!("unable to pop value off an empty stack");
                }
            }
        }
    }

    // # Instruction functions

    /// Adds the value of the src target to the value of the dest target
    fn add(&mut self, dest: Target, src: Target) -> VMResult<()> {
        let dest_value = self.pop(&dest)?;
        let src_value = self.pop(&src)?;

        self.push(dest, dest_value + src_value);

        Ok(())
    }

    /// Subtracts the value of the src target from the value of the dest target
    fn sub(&mut self, dest: Target, src: Target) -> VMResult<()> {
        let dest_value = self.pop(&dest)?;
        let src_value = self.pop(&src)?;

        self.push(dest, dest_value - src_value);

        Ok(())
    }

    /// Divides the value of the dest target through the value of the src target
    fn div(&mut self, dest: Target, src: Target) -> VMResult<()> {
        let dest_value = self.pop(&dest)?;
        let src_value = self.pop(&src)?;

        self.push(dest, dest_value / src_value);

        Ok(())
    }

    /// Multiplies the value of the dest target with the value of the src target
    fn mul(&mut self, dest: Target, src: Target) -> VMResult<()> {
        let dest_value = self.pop(&dest)?;
        let src_value = self.pop(&src)?;

        self.push(dest, dest_value * src_value);

        Ok(())
    }

    /// Applies the modulo operator on the value of the dest target using the
    /// value of the src target
    fn modulo(&mut self, dest: Target, src: Target) -> VMResult<()> {
        let dest_value = self.pop(&dest)?;
        let src_value = self.pop(&src)?;

        self.push(dest, dest_value * src_value);

        Ok(())
    }

    /// Compares the top values of the two targets and saves the result to
    /// `self.cmp_register`
    fn cmp(&mut self, target_a: Target, target_b: Target) -> VMResult<()> {
        self.reset_cmp();

        let target_a_value = self.pop(&target_a)?;
        let target_b_value = self.pop(&target_b)?;

        if target_a_value < target_b_value {
            self.cmp_register = Some(Ordering::Less);
        } else if target_a_value > target_b_value {
            self.cmp_register = Some(Ordering::Greater);
        } else if target_a_value == target_b_value {
            self.cmp_register = Some(Ordering::Equal);
        }

        Ok(())
    }

    /// Jumps unconditionally to the specified address
    fn jmp(&mut self, addr: Address) {
        self.pc = addr;
        self.pc_locked = true;
    }

    /// Jumps if the last compare got the result `Some(Ordering::Less)`
    fn jmp_lt(&mut self, addr: Address) {
        if self.cmp_register == Some(Ordering::Less) {
            self.jmp(addr);
            self.reset_cmp();
        }
    }

    /// Jumps if the last compare got the result `Some(Ordering::Greater)`
    fn jmp_gt(&mut self, addr: Address) {
        if self.cmp_register == Some(Ordering::Greater) {
            self.jmp(addr);
            self.reset_cmp();
        }
    }

    /// Jumps if the last compare got the result `Some(Ordering::Equal)`
    fn jmp_eq(&mut self, addr: Address) {
        if self.cmp_register == Some(Ordering::Equal) {
            self.jmp(addr);
            self.reset_cmp();
        }
    }

    /// Pushes the given value to the given target
    fn push(&mut self, dest: Target, value: Value) {
        match dest {
            Target::Register(index) => self.data_registers[index] = value,
            Target::Stack(index) => self.data_stacks[index].push_front(value),
        }
    }

    /// Moves the top value of the src target to the dest target
    fn mov(&mut self, dest: Target, src: Target) -> VMResult<()> {
        let src_value = self.pop(&src)?;
        self.push(dest, src_value);

        Ok(())
    }

    /// Swaps the top values of the targets
    fn swp(&mut self, target_a: Target, target_b: Target) -> VMResult<()> {
        let a_value = self.pop(&target_a)?;
        let b_value = self.pop(&target_b)?;

        self.push(target_a, b_value);
        self.push(target_b, a_value);

        Ok(())
    }

    /// Calls the function at the specified address saving the return address
    /// to the call stack
    fn call(&mut self, addr: Address) {
        self.call_stack.push_front(self.pc + 1);
        self.jmp(addr);
    }

    /// Returns from an ongoing function call
    fn ret(&mut self) -> VMResult<()> {
        if self.call_stack.is_empty() {
            bail!("unable to return from an empty call stack");
        }

        let retur_addr = self.call_stack.pop_front().unwrap();

        self.jmp(retur_addr);

        Ok(())
    }
}
