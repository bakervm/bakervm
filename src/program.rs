use instruction::Instruction;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

pub type Program = Vec<Instruction>;

pub fn decode<P: AsRef<Path>>(path: P) -> Program {
    let mut f = File::open(path).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    let mut program = Program::new();

    let mut byte_iter = s.bytes();
    let mut byte = byte_iter.next();
    while byte != None {
        let res = match byte {
            Some(byte) => {
                match byte {
                    1 => Instruction::HALT,
                    2 => Instruction::IADD,
                    3 => Instruction::ISUB,
                    4 => Instruction::PRINT,
                    5 => {
                        let mut res: i64 = byte_iter.next().unwrap() as i64;
                        res << 8;
                        res = res & byte_iter.next().unwrap() as i64;
                        res << 8;
                        res = res & byte_iter.next().unwrap() as i64;
                        res << 8;
                        res = res & byte_iter.next().unwrap() as i64;

                        Instruction::PUSH(res)
                    }
                    _ => panic!("Unexpected opcode"),
                }
            }
            None => panic!("Unexpected end of file"),
        };

        program.push(res);

        byte = byte_iter.next();
    }

    program
}
