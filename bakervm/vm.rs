use definitions::{ExternalInterrupt, InternalInterrupt};
use definitions::Config;
use definitions::Instruction;
use definitions::Program;
use definitions::Target;
use definitions::Value;
use definitions::typedef::*;
use error::*;
use std::collections::{BTreeMap, LinkedList};
use std::sync::{Arc, Barrier};
use std::sync::mpsc::{Receiver, SyncSender, TrySendError};
use std::thread::{self, JoinHandle};
use std::time::Instant;

pub fn start(
    program: Program, sender: SyncSender<Frame>, receiver: Receiver<ExternalInterrupt>,
    barrier: Arc<Barrier>
) -> JoinHandle<()> {
    thread::spawn(
        move || {
            barrier.wait();
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

/// Since rusts `std:::cmp::Ordering` doesn't implement serialization, we have
/// to do this
#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum Ordering {
    Less,
    Greater,
    Equal,
}

/// The whole state of the VM
#[derive(Serialize, Deserialize, Default, Debug)]
struct VM {
    /// The instructions that are currently executed
    image_data: Vec<Instruction>,
    /// The current program counter
    pc: Address,
    stack: LinkedList<Value>,
    val_index: BTreeMap<Address, Value>,
    input_register: Integer,
    framebuffer: Frame,
    framebuffer_invalid: bool,
    /// A register for holding information about a recent comparison
    cmp_register: Option<Ordering>,
    /// A stack to hold the return addresses of function calls
    call_stack: LinkedList<Address>,
    /// A boolean used for locking the program counter
    pc_locked: bool,
    /// The configuration of the VM
    config: Config,
    halted: bool,
    paused: bool,
}

impl VM {
    // # Maintainance functions

    /// Executes the given program
    pub fn exec(
        &mut self, program: Program, sender: SyncSender<Frame>,
        receiver: Receiver<ExternalInterrupt>
    ) -> VMResult<()> {
        self.reset();
        self.load_program(program).chain_err(|| "invalid program container")?;
        self.build_framebuffer();

        let mut instruction_count = 0;
        let mut now_before = Instant::now();

        while (self.pc < self.image_data.len()) && !self.halted {
            self.external_interrupt(&receiver)?;

            self.do_cycle()?;

            self.flush_framebuffer(&sender)?;

            thread::yield_now();

            let secs_elapsed = now_before.elapsed().as_secs();

            if secs_elapsed >= 1 {
                println!("IPS: {:?}", instruction_count / secs_elapsed);
                now_before = Instant::now();
                instruction_count = 0;
            }

            instruction_count += 1;
        }

        Ok(())
    }

    /// Run one instruction cycle
    fn do_cycle(&mut self) -> VMResult<()> {
        let current_instruction = self.image_data[self.pc].clone();
        self.handle_instruction(current_instruction)?;
        self.advance_pc();

        Ok(())
    }

    /// Handles a single instruction
    fn handle_instruction(&mut self, instruction: Instruction) -> VMResult<()> {
        match instruction {
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

            Instruction::Halt => self.halt(),
            Instruction::Pause => self.pause(),
            Instruction::Int(interrupt) => self.int(&interrupt),
        }

        Ok(())
    }

    /// Loads the instructions of the given program to the VM's state
    fn load_program(&mut self, program: Program) -> VMResult<()> {
        let orig_program = Program::default();
        if program.preamble != orig_program.preamble {
            bail!("invalid preamble");
        } else if program.version != orig_program.version {
            bail!("invalid version");
        } else {
            self.image_data = program.instructions;
            self.config = program.config;
            Ok(())
        }
    }

    /// Aborts the execution of the current image
    fn halt(&mut self) {
        self.halted = true;
    }

    /// Pauses the execution of the program until an interrupt is received
    fn pause(&mut self) {
        self.paused = true;
    }

    /// Handles an internal interrupt
    fn int(&mut self, interrupt: &InternalInterrupt) {
        match interrupt {
            &InternalInterrupt::FlushFramebuffer => self.invalidate_framebuffer(),
        }
    }

    /// Handles incoming interrupts or moves along
    fn external_interrupt(&mut self, receiver: &Receiver<ExternalInterrupt>) -> VMResult<()> {
        let interrupt = if self.paused {
            self.paused = false;
            if let Ok(interrupt) = receiver.recv() {
                interrupt
            } else {
                return Ok(());
            }
        } else {
            if let Ok(interrupt) = receiver.try_recv() {
                interrupt
            } else {
                return Ok(());
            }
        };

        match interrupt {
            ExternalInterrupt::Halt => self.halt(),
            ExternalInterrupt::KeyDown(value) => {
                self.input_register = value;
            }
            ExternalInterrupt::KeyUp => self.input_register = 0,
            _ => {}
        }

        Ok(())

    }

    /// Flushes the internal framebuffer using the given sender
    fn flush_framebuffer(&mut self, sender: &SyncSender<Frame>) -> VMResult<()> {
        if self.framebuffer_invalid {
            if let Err(TrySendError::Disconnected(..)) = sender.try_send(self.framebuffer.clone()) {
                bail!("output channel disconnected");
            }
            self.framebuffer_invalid = false;
        }

        Ok(())
    }

    /// Allocates all the needed space in the framebuffer
    fn build_framebuffer(&mut self) {
        let ref resolution = self.config.display.resolution;
        let allocation_space = resolution.width * resolution.height;
        self.framebuffer = vec![Color::default(); allocation_space];
    }

    /// Resets the VM to a clean state
    fn reset(&mut self) {
        *self = VM::default();
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
    fn invalidate_framebuffer(&mut self) {
        self.framebuffer_invalid = true;
    }

    /// Return the value at the specified target. The value of the target will
    /// be consumed
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
                if let Some(&(r, g, b)) = self.framebuffer.get(index) {
                    Ok(Value::Color(r, g, b))
                } else {
                    bail!("no value found in framebuffer at index {}", index);
                }
            }
            &Target::InputRegister => Ok(Value::Integer(self.input_register)),
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
        }
    }

    /// Jumps if the last compare got the result `Some(Ordering::Greater)` or
    /// `Some(Ordering::Equal)`
    fn jmp_gt_eq(&mut self, addr: &Address) {
        if (self.cmp_register == Some(Ordering::Greater)) ||
           (self.cmp_register == Some(Ordering::Equal)) {
            self.jmp(addr);
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
                if let Value::Color(r, g, b) = value {
                    self.framebuffer[index] = (r, g, b);
                    Ok(())
                } else {
                    bail!("unable push a non-color value to the framebuffer");
                }
            }
            &Target::InputRegister => {
                bail!("unable push to read-only input register");
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

#[cfg(test)]
mod tests {
    use super::*;
    use definitions::ImageBuilder;
    use rand;

    #[test]
    fn halt() {
        let mut vm = VM::default();
        vm.handle_instruction(Instruction::Halt).unwrap();

        println!("{:#?}", vm);

        assert!(vm.halted);
    }

    #[test]
    fn add_stack() {
        for _ in 0..3000 {
            let val_a = rand::random::<Integer>() / 2;
            let val_b = rand::random::<Integer>() / 2;

            let mut vm = VM::default();

            let mut builder = ImageBuilder::new();
            builder.push(Target::Stack, Value::Integer(val_a));
            builder.push(Target::Stack, Value::Integer(val_b));
            builder.add(Target::Stack, Target::Stack);

            let program = builder.gen_program();
            vm.load_program(program).unwrap();

            vm.do_cycle().unwrap();
            vm.do_cycle().unwrap();
            vm.do_cycle().unwrap();

            let stack_value = vm.pop(&Target::Stack).unwrap();

            assert_eq!(stack_value, Value::Integer(val_b + val_a));
        }
    }

    #[test]
    fn sub_stack() {
        for _ in 0..3000 {
            let val_a = rand::random::<Integer>() / 2;
            let val_b = rand::random::<Integer>() / 2;

            let mut vm = VM::default();

            let mut builder = ImageBuilder::new();
            builder.push(Target::Stack, Value::Integer(val_a));
            builder.push(Target::Stack, Value::Integer(val_b));
            builder.sub(Target::Stack, Target::Stack);

            let program = builder.gen_program();
            vm.load_program(program).unwrap();

            vm.do_cycle().unwrap();
            vm.do_cycle().unwrap();
            vm.do_cycle().unwrap();

            let stack_value = vm.pop(&Target::Stack).unwrap();

            assert_eq!(stack_value, Value::Integer(val_b - val_a));
        }
    }

    #[test]
    fn mul_stack() {
        for _ in 0..3000 {
            let val_a = rand::random::<Float>() % 3037000499.0;
            let val_b = rand::random::<Float>() % 3037000499.0;

            let mut vm = VM::default();

            let mut builder = ImageBuilder::new();
            builder.push(Target::Stack, Value::Float(val_a));
            builder.push(Target::Stack, Value::Float(val_b));
            builder.mul(Target::Stack, Target::Stack);

            let program = builder.gen_program();
            vm.load_program(program).unwrap();

            vm.do_cycle().unwrap();
            vm.do_cycle().unwrap();
            vm.do_cycle().unwrap();

            let stack_value = vm.pop(&Target::Stack).unwrap();

            assert_eq!(stack_value, Value::Float(val_b * val_a));
        }
    }

    #[test]
    fn div_stack() {
        for _ in 0..3000 {
            let val_a = rand::random::<Float>();
            let val_b = rand::random::<Float>();

            let mut vm = VM::default();

            let mut builder = ImageBuilder::new();
            builder.push(Target::Stack, Value::Float(val_a));
            builder.push(Target::Stack, Value::Float(val_b));
            builder.div(Target::Stack, Target::Stack);

            let program = builder.gen_program();
            vm.load_program(program).unwrap();

            vm.do_cycle().unwrap();
            vm.do_cycle().unwrap();
            vm.do_cycle().unwrap();

            let stack_value = vm.pop(&Target::Stack).unwrap();

            assert_eq!(stack_value, Value::Float(val_b / val_a));
        }
    }
}
