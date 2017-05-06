use definitions::bytecode;
use definitions::typedef::*;
use error::*;
use ieee754::Ieee754;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub fn compile(file: File, output: &str) -> CompilationResult<File> {
    let reader = BufReader::new(file);

    let mut program: ImageData = ImageData::new();

    let mut preamble = String::from(bytecode::PREAMBLE).into_bytes();

    program.append(&mut preamble);

    let mut line_addr_table: Vec<Address> = Vec::new();

    for line in reader.lines() {
        let line = line.chain_err(|| "unable to read line")?;

        if line.is_empty() {
            continue;
        }

        if let Some(first_half) = line.split(';').next() {
            let first_half = first_half.trim();

            let mut first_half_split = first_half.split(' ');

            if let Some(opcode) = first_half_split.next() {
                let opcode = opcode.trim().to_lowercase();

                let args: Vec<String> = if let Some(args) = first_half_split.next() {
                    args.split(',').map(|arg| arg.trim().to_lowercase()).collect()
                } else {
                    Vec::new()
                };

                let mut bytes = compile_instruction(opcode, args)
                    .chain_err(|| "unable to handle instruction")?;

                line_addr_table.push(program.len());

                program.append(&mut bytes);
            } else {
                bail!("opcode expected");
            }
        } else {
            bail!("instruction expected");
        }
    }

    let mut output_file = File::create(output).chain_err(|| "unable to create output file")?;

    output_file.write_all(program.as_slice()).chain_err(|| "unable to write to file")?;

    Ok(output_file)
}

fn compile_instruction(opcode: String, args: Vec<String>) -> CompilationResult<ImageData> {
    let mut res = Vec::new();
    match opcode.as_str() {
        "halt" => res.push(bytecode::HALT),
        "add" => res.push(bytecode::ADD),
        "sub" => res.push(bytecode::SUB),
        "mul" => res.push(bytecode::MUL),
        "div" => res.push(bytecode::DIV),
        "push" => res.push(bytecode::PUSH),
        "jmp" => res.push(bytecode::JMP),
        "jz" => res.push(bytecode::JZ),
        "jnz" => res.push(bytecode::JNZ),
        _ => bail!("unknown opcode: {}", opcode),
    }

    match opcode.as_str() {
        "jmp" | "jz" | "jnz" => {
            if args.len() == 1 {
                let number: Word = args[0].parse().chain_err(|| "unable to parse number")?;

                for i in (0..8).rev() {
                    let shift = i * 8;
                    res.push(((number >> shift) & 0xff) as Byte);
                }
            } else {
                bail!("expected 1 argument, got {}", args.len());
            }
        }
        "push" => {
            if args.len() == 1 {
                let float_number: f64 = args[0].parse().chain_err(|| "unable to parse number")?;

                let save_number: Word = float_number.bits();

                for i in (0..8).rev() {
                    let shift = i * 8;
                    res.push(((save_number >> shift) & 0xff) as Byte);
                }
            } else {
                bail!("expected 1 argument, got {}", args.len());
            }
        }
        _ => {}
    }

    Ok(res)
}
