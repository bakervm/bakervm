use definitions::{Config, ExternalInterrupt, Instruction, InternalInterrupt, Program, Target,
                  Type, Value};
use definitions::error::*;
use definitions::typedef::*;
use std::collections::{BTreeMap, LinkedList};
use std::sync::{Arc, Barrier};
use std::sync::mpsc::{Receiver, SyncSender, TrySendError};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

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

                ::std::process::exit(1);
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

const NUM_RESERVED_MEM_SLOTS: usize = 20;

// const FRAMEBUFFER_CURSOR_INDEX: Target = Target::ValueIndex(0);
const CURRENT_KEYCODE_INDEX: Target = Target::ValueIndex(1);
const DISPLAY_WIDTH_INDEX: Target = Target::ValueIndex(2);
const DISPLAY_HEIGHT_INDEX: Target = Target::ValueIndex(3);
const MOUSE_BUTTON_INDEX: Target = Target::ValueIndex(4);
const MOUSE_X_INDEX: Target = Target::ValueIndex(5);
const MOUSE_Y_INDEX: Target = Target::ValueIndex(6);

/// The whole state of the VM
#[derive(Serialize, Deserialize, Default, Debug)]
struct VM {
    /// The instructions that are currently executed
    image_data: Vec<Instruction>,
    /// The current program counter
    pc: Address,
    base_ptr: Address,
    stack: LinkedList<Value>,
    value_index: BTreeMap<Address, Value>,
    framebuffer: Frame,
    framebuffer_invalid: bool,
    next_frame: Frame,
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
    ) -> Result<()> {
        self.reset();
        self.load_program(&program).chain_err(|| "invalid program container")?;
        self.build_framebuffer();

        self.push(
                &DISPLAY_WIDTH_INDEX,
                Value::Address(program.config.display.resolution.width.clone()),
            )?;

        self.push(
                &DISPLAY_HEIGHT_INDEX,
                Value::Address(program.config.display.resolution.height.clone()),
            )?;

        let mut now = Instant::now();

        while (self.pc < self.image_data.len()) && !self.halted {
            self.do_cycle()?;

            let elapsed_time = now.elapsed();
            if self.framebuffer_invalid && elapsed_time >= Duration::from_millis(16) {
                let res = sender.try_send(self.next_frame.clone());
                if let Err(TrySendError::Disconnected(..)) = res {
                    self.halt();
                } else if let Ok(()) = res {
                    self.framebuffer_invalid = false;
                    now = Instant::now();
                }
            }

            self.external_interrupt(&receiver, &sender)?;
        }

        Ok(())
    }

    /// Returns the instruction at the current program counter
    fn current_instruction(&mut self) -> Result<Instruction> {
        if let Some(current_instruction) = self.image_data.get(self.pc) {
            Ok(current_instruction.clone())
        } else {
            bail!("no instruction found at index {}", self.pc);
        }
    }

    /// Run one instruction cycle
    fn do_cycle(&mut self) -> Result<()> {
        let current_instruction = self.current_instruction()?;

        self.handle_instruction(current_instruction)?;
        self.advance_pc();

        Ok(())
    }

    /// Handles a single instruction
    fn handle_instruction(&mut self, instruction: Instruction) -> Result<()> {
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

            Instruction::Cast(target, val_type) => self.cast(&target, &val_type)?,

            Instruction::Push(dest, value) => self.push(&dest, value)?,
            Instruction::Mov(dest, src) => self.mov(&dest, &src)?,
            Instruction::Swp(target_a, target_b) => self.swp(&target_a, &target_b)?,
            Instruction::Dup(target) => self.dup(&target)?,

            Instruction::Call(addr) => self.call(&addr),
            Instruction::Ret => self.ret()?,

            Instruction::Halt => self.halt(),
            Instruction::Pause => self.pause(),
            Instruction::Nop => {}
            Instruction::Int(interrupt) => self.int(&interrupt),
        }

        Ok(())
    }

    /// Loads the instructions of the given program to the VM's state
    fn load_program(&mut self, program: &Program) -> Result<()> {
        let orig_program = Program::default();
        if program.preamble != orig_program.preamble {
            bail!("invalid preamble");
        } else if program.version != orig_program.version {
            bail!("invalid version");
        } else {
            self.image_data = program.instructions.clone();
            self.config = program.config.clone();

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
            &InternalInterrupt::FlushFramebuffer => {
                self.next_frame = self.framebuffer.clone();
                self.invalidate_framebuffer();
            }
        }
    }

    /// Handles incoming interrupts or moves along
    fn external_interrupt(&mut self, receiver: &Receiver<ExternalInterrupt>, sender: &SyncSender<Frame>)
        -> Result<()> {
        let interrupt = if self.paused {
            self.paused = false;
            // We don't know how long this is going to take... better tell I/O what's going
            // on
            self.wait_flush_framebuffer(sender);
            if let Ok(interrupt) = receiver.recv() {
                interrupt
            } else {
                self.halt();
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
                self.push(&CURRENT_KEYCODE_INDEX, Value::Address(value))?;
            }
            ExternalInterrupt::KeyUp => self.push(&CURRENT_KEYCODE_INDEX, Value::Address(0))?,
            ExternalInterrupt::MouseDown { button, x, y } => {
                self.push(&MOUSE_BUTTON_INDEX, Value::Address(button))?;
                self.push(&MOUSE_X_INDEX, Value::Address(x))?;
                self.push(&MOUSE_Y_INDEX, Value::Address(y))?;
            }
            ExternalInterrupt::MouseUp { x, y } => {
                self.push(&MOUSE_BUTTON_INDEX, Value::Address(0))?;
                self.push(&MOUSE_X_INDEX, Value::Address(x))?;
                self.push(&MOUSE_Y_INDEX, Value::Address(y))?;
            }
        }

        Ok(())

    }

    /// Waits for the channel to be available, then flushes the internal
    /// framebuffer using the given sender
    fn wait_flush_framebuffer(&mut self, sender: &SyncSender<Frame>) {
        if self.framebuffer_invalid {
            let res = sender.send(self.next_frame.clone());
            if let Err(..) = res {
                self.halt();
            } else {
                self.framebuffer_invalid = false;
            }
        }
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
        self.base_ptr = NUM_RESERVED_MEM_SLOTS;
    }

    /// Locks the program counter in place
    fn lock_pc(&mut self) {
        self.pc_locked = true;
    }

    fn get_framebuffer_index(&mut self) -> Result<Address> {
        let index = if let &mut Value::Address(addr) =
            self.value_index.entry(0).or_insert(Value::Address(0)) {
            addr
        } else {
            bail!("unable to access a non-address index");
        };

        Ok(index as Address)
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

    /// Calculates the internal index
    fn internal_index(&mut self, index: Address) -> Address {
        self.base_ptr - (index - NUM_RESERVED_MEM_SLOTS)
    }

    /// Return the value at the specified target. The value of the target will
    /// be consumed
    fn pop(&mut self, target: &Target) -> Result<Value> {
        match target {
            &Target::ValueIndex(index) => {
                if index < NUM_RESERVED_MEM_SLOTS {
                    if let Some(value) = self.value_index.remove(&index) {
                        Ok(value)
                    } else {
                        bail!("no value found at system-reserved index {}", index);
                    }
                } else {
                    let bp_enhanced_index = self.internal_index(index);

                    if bp_enhanced_index < NUM_RESERVED_MEM_SLOTS {
                        bail!("cannot access value without further allocation");
                    }

                    if let Some(value) = self.value_index.remove(&bp_enhanced_index) {
                        Ok(value)
                    } else {
                        bail!("no value found at index {}", bp_enhanced_index);
                    }
                }
            }
            &Target::Stack => {
                if let Some(value) = self.stack.pop_front() {
                    Ok(value)
                } else {
                    bail!("unable to pop value off an empty stack");
                }
            }
            &Target::Framebuffer => {
                let index = self.get_framebuffer_index()?;

                if let Some(&(r, g, b)) = self.framebuffer.get(index) {
                    Ok(Value::Color(r, g, b))
                } else {
                    bail!("no value found in framebuffer at index {}", index);
                }
            }
            &Target::BasePointer => Ok(Value::Address(self.base_ptr)),
        }
    }

    // # Instruction functions

    /// Adds the value of the src target to the value of the dest target
    fn add(&mut self, dest: &Target, src: &Target) -> Result<()> {
        let dest_value = self.pop(dest)?;
        let src_value = self.pop(src)?;

        self.push(dest, (dest_value + src_value)?)?;

        Ok(())
    }

    /// Subtracts the value of the src target from the value of the dest target
    fn sub(&mut self, dest: &Target, src: &Target) -> Result<()> {
        let dest_value = self.pop(dest)?;
        let src_value = self.pop(src)?;

        self.push(dest, (dest_value - src_value)?)?;

        Ok(())
    }

    /// Divides the value of the dest target through the value of the src target
    fn div(&mut self, dest: &Target, src: &Target) -> Result<()> {
        let dest_value = self.pop(dest)?;
        let src_value = self.pop(src)?;

        self.push(dest, (dest_value / src_value)?)?;

        Ok(())
    }

    /// Multiplies the value of the dest target with the value of the src target
    fn mul(&mut self, dest: &Target, src: &Target) -> Result<()> {
        let dest_value = self.pop(dest)?;
        let src_value = self.pop(src)?;

        self.push(dest, (dest_value * src_value)?)?;

        Ok(())
    }

    /// Applies the modulo operator on the value of the dest target using the
    /// value of the src target
    fn rem(&mut self, dest: &Target, src: &Target) -> Result<()> {
        let dest_value = self.pop(dest)?;
        let src_value = self.pop(src)?;

        self.push(dest, (dest_value * src_value)?)?;

        Ok(())
    }

    /// Compares the top values of the two targets and saves the result to
    /// `self.cmp_register`
    fn cmp(&mut self, target_a: &Target, target_b: &Target) -> Result<()> {
        let target_a_value = self.pop(target_a)?;
        let target_b_value = self.pop(target_b)?;

        if target_a_value < target_b_value {
            self.cmp_register = Some(Ordering::Less);
        } else if target_a_value > target_b_value {
            self.cmp_register = Some(Ordering::Greater);
        } else if target_a_value == target_b_value {
            self.cmp_register = Some(Ordering::Equal);
        }

        self.push(target_a, target_a_value)?;
        self.push(target_b, target_b_value)?;

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

    /// Casts a value in-place to the specified type
    fn cast(&mut self, target: &Target, val_type: &Type) -> Result<()> {
        let value = self.pop(target)?;

        let new_value = value.convert_to(val_type);

        self.push(target, new_value)?;

        Ok(())
    }

    /// Pushes the given value to the given target
    fn push(&mut self, dest: &Target, value: Value) -> Result<()> {
        match dest {
            &Target::ValueIndex(index) => {
                if index < NUM_RESERVED_MEM_SLOTS {
                    let mut index_value =
                        self.value_index.entry(index).or_insert(Value::Address(0));
                    *index_value = value;
                } else {
                    let bp_enhanced_index = self.internal_index(index);

                    if bp_enhanced_index < NUM_RESERVED_MEM_SLOTS {
                        bail!("cannot access value without further allocation");
                    }

                    let mut index_value =
                        self.value_index.entry(bp_enhanced_index).or_insert(Value::Address(0));
                    *index_value = value;
                }

                Ok(())
            }
            &Target::Stack => {
                self.stack.push_front(value);
                Ok(())
            }
            &Target::Framebuffer => {
                let index = self.get_framebuffer_index()?;

                if index >= self.framebuffer.len() {
                    bail!("framebuffer index {:?} is out of bounds", index);
                }

                if let Value::Color(r, g, b) = value {
                    self.framebuffer[index] = (r, g, b);
                    Ok(())
                } else {
                    bail!("unable push a non-color value to the framebuffer");
                }
            }
            &Target::BasePointer => {
                if let Value::Address(addr) = value {
                    self.base_ptr = addr;
                    Ok(())
                } else {
                    bail!("unable push a non-address value to the framebuffer");
                }
            }
        }
    }

    /// Moves the top value of the src target to the dest target
    fn mov(&mut self, dest: &Target, src: &Target) -> Result<()> {
        let src_value = self.pop(src)?;
        self.push(dest, src_value)?;

        Ok(())
    }

    /// Swaps the top values of the targets
    fn swp(&mut self, target_a: &Target, target_b: &Target) -> Result<()> {
        let a_value = self.pop(target_a)?;
        let b_value = self.pop(target_b)?;

        self.push(target_a, b_value)?;
        self.push(target_b, a_value)?;

        Ok(())
    }

    fn dup(&mut self, target: &Target) -> Result<()> {
        let value = self.pop(target)?;

        self.push(&Target::Stack, value.clone())?;

        self.push(target, value)?;

        Ok(())
    }

    /// Calls the function at the specified address saving the return address
    /// to the call stack
    fn call(&mut self, addr: &Address) {
        self.call_stack.push_front(self.pc + 1);
        self.jmp(addr);
    }

    /// Returns from an ongoing function call
    fn ret(&mut self) -> Result<()> {
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
    use bincode;
    use definitions::ImageBuilder;
    use rand;

    #[test]
    fn halt() {
        let mut vm = VM::default();
        vm.handle_instruction(Instruction::Halt).unwrap();

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
            vm.load_program(&program).unwrap();

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
            vm.load_program(&program).unwrap();

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
            let val_a = rand::random::<Float>() / 2.0;
            let val_b = rand::random::<Float>() / 2.0;

            let mut vm = VM::default();

            let mut builder = ImageBuilder::new();
            builder.push(Target::Stack, Value::Float(val_a));
            builder.push(Target::Stack, Value::Float(val_b));
            builder.mul(Target::Stack, Target::Stack);

            let program = builder.gen_program();
            vm.load_program(&program).unwrap();

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
            vm.load_program(&program).unwrap();

            vm.do_cycle().unwrap();
            vm.do_cycle().unwrap();
            vm.do_cycle().unwrap();

            let stack_value = vm.pop(&Target::Stack).unwrap();

            assert_eq!(stack_value, Value::Float(val_b / val_a));
        }
    }

    #[test]
    fn load_stock_image() {
        let program_data = include_bytes!("stock.img");

        let program = bincode::deserialize(program_data).unwrap();

        let mut vm = VM::default();

        if let Err(err) = vm.load_program(&program) {
            panic!("program loading failed: {:?}", err);
        }
    }
}
