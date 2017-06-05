use definitions::ImageBuilder;
use definitions::error::*;
use definitions::typedef::*;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;


pub fn compile(file: File) -> Result<ImageData> {
    let reader = BufReader::new(file);

    let mut builder = ImageBuilder::new();

    for line in reader.lines() {
        let line = line.chain_err(|| "unable to read line")?;

        if line.is_empty() {
            continue;
        }

        if let Some(first_half) = line.rsplitn(2, ';').next() {
            let first_half = first_half.trim();

            let mut first_half_split = first_half.splitn(2, ' ');

            if let Some(opcode) = first_half_split.next() {
                let opcode = opcode.trim().to_lowercase();


                let args: Vec<String> = if let Some(args) = first_half_split.next() {
                    args.split(',').map(|arg| arg.trim().to_owned()).collect()
                } else {
                    Vec::new()
                };

                println!("Compiling {} with args: {:?}", opcode, args);
                compile_instruction(&mut builder, opcode, args)?;
            } else {
                bail!("opcode expected. Found {:?}", first_half_split);
            }
        } else {
            bail!("instruction expected. Found {:?}", line.to_string());
        }
    }

    Ok(builder.gen())
}

fn compile_instruction(builder: &mut ImageBuilder, opcode: String, args: Vec<String>)
    -> Result<()> {
    match opcode.as_str() {
        "add" => builder.add(args[0].parse()?, args[1].parse()?),
        "sub" => builder.sub(args[0].parse()?, args[1].parse()?),
        "div" => builder.div(args[0].parse()?, args[1].parse()?),
        "mul" => builder.mul(args[0].parse()?, args[1].parse()?),
        "rem" => builder.rem(args[0].parse()?, args[1].parse()?),

        "cmp" => builder.cmp(args[0].parse()?, args[1].parse()?),
        "jmp" => builder.jmp(args[0].parse()?),
        "jmplt" => builder.jmp_lt(args[0].parse()?),
        "jmpgt" => builder.jmp_gt(args[0].parse()?),
        "jmpeq" => builder.jmp_eq(args[0].parse()?),
        "jmplteq" => builder.jmp_lt_eq(args[0].parse()?),
        "jmpgteq" => builder.jmp_gt_eq(args[0].parse()?),

        "cast" => builder.cast(args[0].parse()?, args[1].parse()?),
        "push" => builder.push(args[0].parse()?, args[1].parse()?),
        "mov" => builder.mov(args[0].parse()?, args[1].parse()?),
        "swp" => builder.swp(args[0].parse()?, args[1].parse()?),
        "dup" => builder.dup(args[0].parse()?),

        "call" => builder.call(args[0].parse()?),
        "ret" => builder.ret(),

        "halt" => builder.halt(),
        "pause" => builder.pause(),
        "nop" => builder.nop(),
        "int" => builder.int(args[0].parse()?),
        _ => {}
    }

    Ok(())
}
