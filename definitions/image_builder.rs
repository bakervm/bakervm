use bincode::{self, Infinite};
use program::{Instruction, Program, Target};
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

    pub fn add(mut self, dest: Target, src: Target) -> Self {
        self.add_instruction(Instruction::Add(dest, src));
        self
    }

    pub fn sub(mut self, dest: Target, src: Target) -> Self {
        self.add_instruction(Instruction::Sub(dest, src));
        self
    }

    pub fn div(mut self, dest: Target, src: Target) -> Self {
        self.add_instruction(Instruction::Div(dest, src));
        self
    }

    pub fn mul(mut self, dest: Target, src: Target) -> Self {
        self.add_instruction(Instruction::Mul(dest, src));
        self
    }

    pub fn rem(mut self, dest: Target, src: Target) -> Self {
        self.add_instruction(Instruction::Rem(dest, src));
        self
    }

    pub fn cmp(mut self, target_a: Target, target_b: Target) -> Self {
        self.add_instruction(Instruction::Cmp(target_a, target_b));
        self
    }

    pub fn jmp(mut self, addr: Address) -> Self {
        self.add_instruction(Instruction::Jmp(addr));
        self
    }

    pub fn jmp_lt(mut self, addr: Address) -> Self {
        self.add_instruction(Instruction::JmpLt(addr));
        self
    }

    pub fn jmp_gt(mut self, addr: Address) -> Self {
        self.add_instruction(Instruction::JmpGt(addr));
        self
    }

    pub fn jmp_eq(mut self, addr: Address) -> Self {
        self.add_instruction(Instruction::JmpEq(addr));
        self
    }

    pub fn push(mut self, dest: Target, value: Value) -> Self {
        self.add_instruction(Instruction::Push(dest, value));
        self
    }

    pub fn mov(mut self, dest: Target, src: Target) -> Self {
        self.add_instruction(Instruction::Mov(dest, src));
        self
    }

    pub fn swp(mut self, src_a: Target, src_b: Target) -> Self {
        self.add_instruction(Instruction::Swp(src_a, src_b));
        self
    }

    pub fn call(mut self, addr: Address) -> Self {
        self.add_instruction(Instruction::Call(addr));
        self
    }

    pub fn ret(mut self) -> Self {
        self.add_instruction(Instruction::Ret);
        self
    }

    pub fn halt(mut self) -> Self {
        self.add_instruction(Instruction::Halt);
        self
    }

    pub fn gen(self) -> ImageData {
        let program = Program {
            instructions: self.instructions,
            ..Program::default()
        };

        bincode::serialize(&program, Infinite).expect("unable to encode program")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate() {
        let data = ImageBuilder::new()
            .push(Target::Stack, Value::Float(23.0))
            .push(Target::ValueIndex(0), Value::Float(35.0))
            .add(Target::Stack, Target::ValueIndex(0))
            .gen();
    }
}
