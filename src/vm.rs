use instruction::Instruction;
use program::Program;

pub struct VM {
    ip: usize,
    sp: usize,
    code: Program,
    globals: Vec<u64>,
    stack: Vec<u64>,
}

impl VM {
    pub fn new(code: Program, startip: usize) -> VM {
        VM {
            ip: startip,
            sp: 0,
            code: code,
            globals: Vec::new(),
            stack: Vec::new(),
        }
    }

    pub fn exec(&mut self) {
        while self.ip < self.code.len() {
            let bytecode = self.code[self.ip].clone();
            match bytecode {
                Instruction::PUSH(num) => self.push(num),
                Instruction::IADD => self.iadd(),
                Instruction::ISUB => self.isub(),
                Instruction::PRINT => self.print(),
                Instruction::HALT => break,
                _ => panic!("Unknown instruction: {:#?}", bytecode),
            }

            self.ip += 1;
        }
    }

    fn push(&mut self, value: u64) {
        if !self.stack.is_empty() {
            self.sp += 1;
        }

        self.stack.push(value);
    }

    fn pop(&mut self) -> u64 {
        assert!(!self.stack.is_empty(),
                "Unable to pop value off an empty Stack");
        let res = self.stack.remove(self.sp);
        if !self.stack.is_empty() {
            self.sp -= 1;
        }
        res
    }

    fn iadd(&mut self) {
        let b = self.pop();
        let a = self.pop();
        self.push(a + b);
    }

    fn isub(&mut self) {
        let b = self.pop();
        let a = self.pop();
        self.push(a - b);
    }

    fn print(&mut self) {
        print!("{:?}", self.stack[self.sp]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use instruction::Instruction;
    use program::Program;

    #[test]
    fn stack_size() {
        let prog: Program = vec![Instruction::PUSH(234),
                                 Instruction::PUSH(234),
                                 Instruction::PUSH(234),
                                 Instruction::PUSH(234),
                                 Instruction::PUSH(234)];

        let mut vm = VM::new(prog, 0);

        vm.exec();

        assert_eq!(vm.stack.len(), 5);
        assert_eq!(vm.sp, 4);
    }

    #[test]
    fn printer() {
        let prog: Program = vec![Instruction::PUSH(321), Instruction::PUSH(123), Instruction::IADD];

        let mut vm = VM::new(prog, 0);

        vm.exec();
    }
}
