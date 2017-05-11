use bincode::{self, Infinite};
use program::*;
use typedef::*;

pub struct ImageBuilder {
    preamble: String,
    version: String,
    instructions: Vec<Instruction>,
}

impl ImageBuilder {
    pub fn new(preamble: &str, version: &str) -> ImageBuilder {
        ImageBuilder {
            preamble: preamble.to_owned(),
            version: version.to_owned(),
            instructions: Vec::new(),
        }
    }

    fn add_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    pub fn halt(mut self) -> Self {
        self.add_instruction(Instruction::Halt);
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

    pub fn add(mut self, dest: Target, src: Target) -> Self {
        self.add_instruction(Instruction::Add(dest, src));
        self
    }

    pub fn sub(mut self, dest: Target, src: Target) -> Self {
        self.add_instruction(Instruction::Sub(dest, src));
        self
    }

    pub fn mul(mut self, dest: Target, src: Target) -> Self {
        self.add_instruction(Instruction::Mul(dest, src));
        self
    }

    pub fn div(mut self, dest: Target, src: Target) -> Self {
        self.add_instruction(Instruction::Div(dest, src));
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

    pub fn jmp(mut self, addr: Address) -> Self {
        self.add_instruction(Instruction::Jmp(addr));
        self
    }

    pub fn gen(self) -> ImageData {
        let program = Program {
            preamble: self.preamble,
            version: self.version,
            instructions: self.instructions,
        };

        bincode::serialize(&program, Infinite).expect("unable to encode program")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate() {
        let data = ImageBuilder::new("BAKER", "0.3.0")
            .push(Target::Stack(0), Value::Number(23.0))
            .push(Target::Register(0), Value::Number(35.0))
            .add(Target::Stack(0), Target::Register(0))
            .gen();
    }
}
