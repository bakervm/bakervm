use bincode::{self, Infinite};
use instruction::Instruction;
use interrupt::InternalInterrupt;
use program::Program;
use signal::Signal;
use target::Target;
use typedef::*;
use value::Value;

pub struct ImageBuilder {
    instructions: Vec<Instruction>,
}

impl ImageBuilder {
    pub fn new() -> ImageBuilder {
        ImageBuilder { instructions: Vec::new() }
    }

    fn add_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    pub fn add(&mut self, dest: Target, src: Target) {
        self.add_instruction(Instruction::Add(dest, src));
    }

    pub fn sub(&mut self, dest: Target, src: Target) {
        self.add_instruction(Instruction::Sub(dest, src));
    }

    pub fn div(&mut self, dest: Target, src: Target) {
        self.add_instruction(Instruction::Div(dest, src));
    }

    pub fn mul(&mut self, dest: Target, src: Target) {
        self.add_instruction(Instruction::Mul(dest, src));
    }

    pub fn rem(&mut self, dest: Target, src: Target) {
        self.add_instruction(Instruction::Rem(dest, src));
    }

    pub fn cmp(&mut self, target_a: Target, target_b: Target) {
        self.add_instruction(Instruction::Cmp(target_a, target_b));
    }

    pub fn jmp(&mut self, addr: Address) {
        self.add_instruction(Instruction::Jmp(addr));
    }

    pub fn jmp_lt(&mut self, addr: Address) {
        self.add_instruction(Instruction::JmpLt(addr));
    }

    pub fn jmp_gt(&mut self, addr: Address) {
        self.add_instruction(Instruction::JmpGt(addr));
    }

    pub fn jmp_eq(&mut self, addr: Address) {
        self.add_instruction(Instruction::JmpEq(addr));
    }

    pub fn push(&mut self, dest: Target, value: Value) {
        self.add_instruction(Instruction::Push(dest, value));
    }

    pub fn mov(&mut self, dest: Target, src: Target) {
        self.add_instruction(Instruction::Mov(dest, src));
    }

    pub fn swp(&mut self, src_a: Target, src_b: Target) {
        self.add_instruction(Instruction::Swp(src_a, src_b));
    }

    pub fn call(&mut self, addr: Address) {
        self.add_instruction(Instruction::Call(addr));
    }

    pub fn ret(&mut self) {
        self.add_instruction(Instruction::Ret);
    }

    pub fn halt(&mut self) {
        self.add_instruction(Instruction::Halt);
    }

    pub fn int(&mut self, interrupt: InternalInterrupt) {
        self.add_instruction(Instruction::Int(interrupt));
    }

    pub fn ext(&mut self, signal: Signal, addr: Address) {
        self.add_instruction(Instruction::Ext(signal, addr));
    }

    pub fn gen(self) -> ImageData {
        let program = self.gen_program();

        bincode::serialize(&program, Infinite).expect("unable to encode program")
    }

    pub fn gen_program(&self) -> Program {
        Program {
            instructions: self.instructions.clone(),
            ..Program::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate() {
        let mut builder = ImageBuilder::new();
        builder.push(Target::Stack, Value::Float(23.0));
        builder.push(Target::ValueIndex(0), Value::Float(35.0));
        builder.add(Target::Stack, Target::ValueIndex(0));
        builder.gen();
    }
}
