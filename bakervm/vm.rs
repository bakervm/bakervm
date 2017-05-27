use definitions::Value;
use definitions::program::{Instruction, Interrupt, PREAMBLE, Program, Target, VMConfig};
use definitions::typedef::*;
use error::*;
use std::cmp::Ordering;
use std::collections::{BTreeMap, LinkedList};
use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use std::thread::{self, JoinHandle};

pub fn start(program: Program, sender: Sender<Frame>, receiver: Receiver<Interrupt>)
    -> JoinHandle<()> {
    thread::spawn(
        move || {
            if let Err(ref e) = VM::default().exec(program, sender, receiver) {
                println!("error: {}", e);

                for e in e.iter().skip(1) {
                    println!("caused by: {}", e);
                }

                // The backtrace is not always generated. Try to run this example
                // with `RUST_BACKTRACE=1`.
                if let Some(backtrace) = e.backtrace() {
                    println!("backtrace: {:?}", backtrace);
                }
            }
        },
    )
}

/// The whole state of the VM
#[derive(Default, Debug)]
pub struct VM {
    /// The instructions that are currently executed
    image_data: Vec<Instruction>,
    /// The current program counter
    pc: Address,
    stack: LinkedList<Value>,
    val_index: BTreeMap<Address, Value>,
    interrupt_register: BTreeMap<usize, Address>,
    framebuffer: Frame,
    framebuffer_invalid: bool,
    /// A register for holding infomation about a recent comparison
    cmp_register: Option<Ordering>,
    /// A stack to hold the return addresses of function calls
    call_stack: LinkedList<Address>,
    /// A boolean used for locking the program counter
    pc_locked: bool,
    /// The configuration of the VM
    config: VMConfig,
    halted: bool,
}

impl VM {
    // # Maintainance functions

    /// Executes the given program
    pub fn exec(&mut self, program: Program, sender: Sender<Frame>, receiver: Receiver<Interrupt>)
        -> VMResult<()> {
        self.reset();
        self.load_program(program)?;
        self.build_framebuffer();

        while (self.pc < self.image_data.len()) && !self.halted {
            self.handle_interrupts(&receiver)?;

            let current_instruction = self.image_data[self.pc].clone();

            match current_instruction {
                Instruction::Halt => break,

                Instruction::Add(dest, src) => self.add(&dest, &src)?,
                Instruction::Sub(dest, src) => self.sub(&dest, &src)?,
                Instruction::Div(dest, src) => self.div(&dest, &src)?,
                Instruction::Mul(dest, src) => self.mul(&dest, &src)?,
                Instruction::Rem(dest, src) => self.rem(&dest, &src)?,

                Instruction::Cmp(target_a, target_b) => self.cmp(&target_a, &target_b)?,
                Instruction::Jmp(addr) => self.jmp(&addr),
                Instruction::JmpLt(addr) => self.jmp_lt(&addr),
                Instruction::JmpGt(addr) => self.jmp_gt(&addr),
                Instruction::JmpEq(addr) => self.jmp_eq(&addr),
                Instruction::JmpLtEq(addr) => self.jmp_lt_eq(&addr),
                Instruction::JmpGtEq(addr) => self.jmp_gt_eq(&addr),

                Instruction::Push(dest, value) => self.push(&dest, value)?,
                Instruction::Mov(dest, src) => self.mov(&dest, &src)?,
                Instruction::Swp(target_a, target_b) => self.swp(&target_a, &target_b)?,

                Instruction::Call(addr) => self.call(&addr),
                Instruction::Ret => self.ret()?,
            }

            self.send_framebuffer(&sender)?;

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
            self.config = program.config;
            Ok(())
        }
    }

    /// Aborts the execution of the current image
    fn abort(&mut self) {
        self.halted = true;
    }

    /// Handles incoming interrupts or moves along
    fn handle_interrupts(&mut self, receiver: &Receiver<Interrupt>) -> VMResult<()> {
        match receiver.try_recv() {
            Ok(interrupt) => {
                if interrupt.signal_id == 0 {
                    self.abort();
                    return Ok(());
                }

                let call_addr = if let Some(call_addr) = self.interrupt_register
                       .get(&interrupt.signal_id) {
                    call_addr.clone()
                } else {
                    bail!(
                        "no registered interrupt found at signal_id {}",
                        &interrupt.signal_id
                    );
                };

                for value in interrupt.args {
                    self.push(&Target::Stack, value)?;
                }

                self.call(&call_addr);

                Ok(())
            }
            Err(TryRecvError::Disconnected) => bail!("interrupt receiver disconnected"),
            _ => Ok(()),
        }
    }

    /// Sends the internal framebuffer using the given sender
    fn send_framebuffer(&mut self, sender: &Sender<Frame>) -> VMResult<()> {
        if self.framebuffer_invalid {
            sender.send(self.framebuffer.clone()).chain_err(|| "unable to send framebuffer")?;

            self.framebuffer_invalid = false;
        }

        Ok(())
    }

    /// Allocates all the needed space in the framebuffer
    fn build_framebuffer(&mut self) {
        let ref resolution = self.config.display_resolution;
        let allocation_space = resolution.width * resolution.height;
        self.framebuffer = vec![0; allocation_space];
    }

    /// Resets the VM to a clean state
    fn reset(&mut self) {
        *self = VM::default();
    }

    /// Resets the result of the last comparison
    fn reset_cmp(&mut self) {
        self.cmp_register = None;
    }

    /// Locks the program counter in place
    fn lock_pc(&mut self) {
        self.pc_locked = true;
    }

    /// Advances the program counter
    fn advance_pc(&mut self) {
        if self.pc_locked {
            self.pc_locked = false;
        } else {
            self.pc += 1;
        }
    }

    /// Invalidates the frambuffer causing it to be resent to the display
    /// receiver
    fn invalidate(&mut self) {
        self.framebuffer_invalid = true;
    }

    /// Return the value at the specified target. The value will be consumed
    /// from the target.
    fn pop(&mut self, target: &Target) -> VMResult<Value> {
        match target {
            &Target::ValueIndex(index) => {
                if let Some(value) = self.val_index.remove(&index) {
                    Ok(value)
                } else {
                    bail!("no value found at index {}", index);
                }
            }
            &Target::Stack => {
                if let Some(value) = self.stack.pop_front() {
                    Ok(value)
                } else {
                    bail!("unable to pop value off an empty stack");
                }
            }
            &Target::Framebuffer(index) => {
                let res = if let Some(value) = self.framebuffer.get(index) {
                    Ok(Value::Color(*value))
                } else {
                    bail!("no value found in framebuffer at index {}", index);
                };

                self.invalidate();

                res
            }
        }
    }

    // # Instruction functions

    /// Adds the value of the src target to the value of the dest target
    fn add(&mut self, dest: &Target, src: &Target) -> VMResult<()> {
        let dest_value = self.pop(dest)?;
        let src_value = self.pop(src)?;

        self.push(dest, dest_value + src_value)?;

        Ok(())
    }

    /// Subtracts the value of the src target from the value of the dest target
    fn sub(&mut self, dest: &Target, src: &Target) -> VMResult<()> {
        let dest_value = self.pop(dest)?;
        let src_value = self.pop(src)?;

        self.push(dest, dest_value - src_value)?;

        Ok(())
    }

    /// Divides the value of the dest target through the value of the src target
    fn div(&mut self, dest: &Target, src: &Target) -> VMResult<()> {
        let dest_value = self.pop(dest)?;
        let src_value = self.pop(src)?;

        self.push(dest, dest_value / src_value)?;

        Ok(())
    }

    /// Multiplies the value of the dest target with the value of the src target
    fn mul(&mut self, dest: &Target, src: &Target) -> VMResult<()> {
        let dest_value = self.pop(dest)?;
        let src_value = self.pop(src)?;

        self.push(dest, dest_value * src_value)?;

        Ok(())
    }

    /// Applies the modulo operator on the value of the dest target using the
    /// value of the src target
    fn rem(&mut self, dest: &Target, src: &Target) -> VMResult<()> {
        let dest_value = self.pop(dest)?;
        let src_value = self.pop(src)?;

        self.push(dest, dest_value * src_value)?;

        Ok(())
    }

    /// Compares the top values of the two targets and saves the result to
    /// `self.cmp_register`
    fn cmp(&mut self, target_a: &Target, target_b: &Target) -> VMResult<()> {
        self.reset_cmp();

        let target_a_value = self.pop(target_a)?;
        let target_b_value = self.pop(target_b)?;

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
    fn jmp(&mut self, addr: &Address) {
        self.pc = *addr;
        self.lock_pc();
    }

    /// Jumps if the last compare got the result `Some(Ordering::Less)`
    fn jmp_lt(&mut self, addr: &Address) {
        if self.cmp_register == Some(Ordering::Less) {
            self.jmp(addr);
        }
    }

    /// Jumps if the last compare got the result `Some(Ordering::Greater)`
    fn jmp_gt(&mut self, addr: &Address) {
        if self.cmp_register == Some(Ordering::Greater) {
            self.jmp(addr);
        }
    }

    /// Jumps if the last compare got the result `Some(Ordering::Equal)`
    fn jmp_eq(&mut self, addr: &Address) {
        if self.cmp_register == Some(Ordering::Equal) {
            self.jmp(addr);
        }
    }

    /// Jumps if the last compare got the result `Some(Ordering::Less)` or
    /// `Some(Ordering::Equal)`
    fn jmp_lt_eq(&mut self, addr: &Address) {
        if (self.cmp_register == Some(Ordering::Less)) ||
           (self.cmp_register == Some(Ordering::Equal)) {
            self.jmp(addr);
            self.reset_cmp();
        }
    }

    /// Jumps if the last compare got the result `Some(Ordering::Greater)` or
    /// `Some(Ordering::Equal)`
    fn jmp_gt_eq(&mut self, addr: &Address) {
        if (self.cmp_register == Some(Ordering::Greater)) ||
           (self.cmp_register == Some(Ordering::Equal)) {
            self.jmp(addr);
            self.reset_cmp();
        }
    }

    /// Pushes the given value to the given target
    fn push(&mut self, dest: &Target, value: Value) -> VMResult<()> {
        match dest {
            &Target::ValueIndex(index) => {
                self.val_index.entry(index).or_insert(value);
                Ok(())
            }
            &Target::Stack => {
                self.stack.push_front(value);
                Ok(())
            }
            &Target::Framebuffer(index) => {
                if let Value::Color(value) = value {
                    self.framebuffer[index] = value;
                    self.invalidate();
                    Ok(())
                } else {
                    bail!("unable push a non-color value to the framebuffer");
                }
            }
        }
    }

    /// Moves the top value of the src target to the dest target
    fn mov(&mut self, dest: &Target, src: &Target) -> VMResult<()> {
        let src_value = self.pop(src)?;
        self.push(dest, src_value)?;

        Ok(())
    }

    /// Swaps the top values of the targets
    fn swp(&mut self, target_a: &Target, target_b: &Target) -> VMResult<()> {
        let a_value = self.pop(target_a)?;
        let b_value = self.pop(target_b)?;

        self.push(target_a, b_value)?;
        self.push(target_b, a_value)?;

        Ok(())
    }

    /// Calls the function at the specified address saving the return address
    /// to the call stack
    fn call(&mut self, addr: &Address) {
        self.call_stack.push_front(self.pc + 1);
        self.jmp(addr);
    }

    /// Returns from an ongoing function call
    fn ret(&mut self) -> VMResult<()> {
        if let Some(retur_addr) = self.call_stack.pop_front() {
            self.jmp(&retur_addr);
        } else {
            bail!("unable to return from an empty call stack");
        }

        Ok(())
    }
}
