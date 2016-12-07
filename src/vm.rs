use instruction::Instruction;
use program::Program;

pub struct VM {
    ip: usize,
    sp: i64,
    code: Program,
    globals: Vec<i64>,
    stack: Vec<i64>,
}

impl VM {
    pub fn new(code: Program, startip: usize) -> VM {
        VM {
            ip: startip,
            sp: -1,
            code: code,
            globals: Vec::new(),
            stack: Vec::new(),
        }
    }

    pub fn exec(&mut self) {
        let mut bytecode = self.code[self.ip].clone();
        while (bytecode != Instruction::HALT) && self.ip < self.code.len() {
            self.ip += 1;
            match bytecode {
                Instruction::PUSH(num) => self.push(num),
                Instruction::IADD => self.iadd(),
                Instruction::ISUB => self.isub(),
                Instruction::PRINT => self.print(),
                _ => panic!("Unknown instruction: {:#?}", bytecode),
            }
            bytecode = self.code[self.ip].clone();
        }
    }

    fn push(&mut self, value: i64) {
        self.stack.push(value);
        self.sp += 1;
    }

    fn pop(&mut self) -> i64 {
        assert!(self.sp >= 0, "Unable to pop value off an empty Stack");
        let res = self.stack.remove(self.sp as usize);
        self.sp -= 1;
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
        println!("{:?}", self.stack[self.sp as usize]);
    }
}
