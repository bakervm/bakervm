use definitions::ImageBuilder;
use definitions::error::*;
use definitions::typedef::*;
use mnemonic::Mnemonic;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

lazy_static! {
    static ref LABELED_MNEMONIC_RE: Regex = Regex::new(r"^\.(.+?) +?(.+)$").unwrap();
    static ref LABEL_RE: Regex = Regex::new(r"^\.([^\s]+)$").unwrap();
}


pub fn compile(file: File) -> Result<ImageData> {
    let reader = BufReader::new(file);

    let mut builder = ImageBuilder::new();

    let mut label_addr_map = HashMap::new();

    let mut mnemonic_vector = Vec::new();

    let mut instruction_count = 0;

    for line in reader.lines() {
        let line = line.chain_err(|| "unable to read line")?;

        if line.is_empty() {
            continue;
        }

        if let Some(first_half) = line.rsplitn(2, ';').next() {
            let first_half = first_half.trim();

            let first_half: String = if LABELED_MNEMONIC_RE.is_match(first_half) {
                let captures =
                    if let Some(captures) = LABELED_MNEMONIC_RE.captures_iter(first_half).next() {
                        captures
                    } else {
                        bail!("no label capture found")
                    };

                let label = captures[1].trim();

                println!("Label {:?} found at address {}", label, instruction_count);

                label_addr_map.entry(label.to_owned()).or_insert(instruction_count);

                captures[2].trim().to_owned()
            } else if LABEL_RE.is_match(first_half) {
                let captures = if let Some(captures) = LABEL_RE.captures_iter(first_half).next() {
                    captures
                } else {
                    bail!("no label capture found")
                };

                let label = captures[1].trim();

                println!("Label {:?} found at address {}", label, instruction_count);

                label_addr_map.entry(label.to_owned()).or_insert(instruction_count);
                continue;
            } else {
                first_half.to_owned()
            };

            println!("Input instruction {:?}", first_half);

            let mut first_half_split = first_half.splitn(2, ' ');

            if let Some(opcode) = first_half_split.next() {
                let opcode = opcode.trim().to_lowercase();


                let args: Vec<String> = if let Some(args) = first_half_split.next() {
                    args.split(',').map(|arg| arg.trim().to_owned()).collect()
                } else {
                    Vec::new()
                };

                println!("Compiling {} with args: {:?}", opcode, args);

                mnemonic_vector.push(text_to_mnemonic(opcode, args)?);

                instruction_count += 1;
            } else {
                bail!("opcode expected. Found {:?}", first_half_split);
            }
        } else {
            bail!("instruction expected. Found {:?}", line.to_string());
        }
    }

    for mnemonic in mnemonic_vector {
        compile_mnemonic(&mut builder, mnemonic.clone(), &label_addr_map)?;
    }

    Ok(builder.gen())
}

fn text_to_mnemonic(opcode: String, args: Vec<String>) -> Result<Mnemonic> {
    match opcode.as_str() {
        "add" => Ok(Mnemonic::Add(args[0].parse()?, args[1].parse()?)),
        "sub" => Ok(Mnemonic::Sub(args[0].parse()?, args[1].parse()?)),
        "div" => Ok(Mnemonic::Div(args[0].parse()?, args[1].parse()?)),
        "mul" => Ok(Mnemonic::Mul(args[0].parse()?, args[1].parse()?)),
        "rem" => Ok(Mnemonic::Rem(args[0].parse()?, args[1].parse()?)),

        "cmp" => Ok(Mnemonic::Cmp(args[0].parse()?, args[1].parse()?)),
        "jmp" => Ok(Mnemonic::Jmp(args[0].parse()?)),
        "jmplt" => Ok(Mnemonic::JmpLt(args[0].parse()?)),
        "jmpgt" => Ok(Mnemonic::JmpGt(args[0].parse()?)),
        "jmpeq" => Ok(Mnemonic::JmpEq(args[0].parse()?)),
        "jmplteq" => Ok(Mnemonic::JmpLtEq(args[0].parse()?)),
        "jmpgteq" => Ok(Mnemonic::JmpGtEq(args[0].parse()?)),

        "cast" => Ok(Mnemonic::Cast(args[0].parse()?, args[1].parse()?)),
        "push" => Ok(Mnemonic::Push(args[0].parse()?, args[1].parse()?)),
        "mov" => Ok(Mnemonic::Mov(args[0].parse()?, args[1].parse()?)),
        "swp" => Ok(Mnemonic::Swp(args[0].parse()?, args[1].parse()?)),
        "dup" => Ok(Mnemonic::Dup(args[0].parse()?)),

        "call" => Ok(Mnemonic::Call(args[0].parse()?)),
        "ret" => Ok(Mnemonic::Ret),

        "halt" => Ok(Mnemonic::Halt),
        "pause" => Ok(Mnemonic::Pause),
        "nop" => Ok(Mnemonic::Nop),
        "int" => Ok(Mnemonic::Int(args[0].parse()?)),
        _ => bail!("unkwnown opcode {:?}", opcode),
    }
}

fn compile_mnemonic(builder: &mut ImageBuilder, mnemonic: Mnemonic, lookup: &HashMap<String, Address>)
    -> Result<()> {

    match mnemonic {
        Mnemonic::Add(dest, src) => builder.add(dest, src),
        Mnemonic::Sub(dest, src) => builder.sub(dest, src),
        Mnemonic::Div(dest, src) => builder.div(dest, src),
        Mnemonic::Mul(dest, src) => builder.mul(dest, src),
        Mnemonic::Rem(dest, src) => builder.rem(dest, src),

        Mnemonic::Cmp(target_a, target_b) => builder.cmp(target_a, target_b),
        Mnemonic::Jmp(label) => builder.jmp(lookup[&label]),
        Mnemonic::JmpLt(label) => builder.jmp_lt(lookup[&label]),
        Mnemonic::JmpGt(label) => builder.jmp_gt(lookup[&label]),
        Mnemonic::JmpEq(label) => builder.jmp_eq(lookup[&label]),
        Mnemonic::JmpLtEq(label) => builder.jmp_lt_eq(lookup[&label]),
        Mnemonic::JmpGtEq(label) => builder.jmp_gt(lookup[&label]),

        Mnemonic::Cast(target, type_t) => builder.cast(target, type_t),

        Mnemonic::Push(target, value) => builder.push(target, value),
        Mnemonic::Mov(dest, src) => builder.mov(dest, src),
        Mnemonic::Swp(target_a, target_b) => builder.swp(target_a, target_b),
        Mnemonic::Dup(target) => builder.dup(target),

        Mnemonic::Call(label) => builder.call(lookup[&label]),
        Mnemonic::Ret => builder.ret(),

        Mnemonic::Halt => builder.halt(),
        Mnemonic::Pause => builder.pause(),
        Mnemonic::Nop => builder.nop(),
        Mnemonic::Int(int) => builder.int(int),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn labeled_mnemonic_regex() {
        let input = ".start mov $st, $vi(19)";

        if !LABELED_MNEMONIC_RE.is_match(input) {
            panic!("input doesn't match a labeled mnemonic");
        } else {
            let captures = LABELED_MNEMONIC_RE.captures_iter(input).next().unwrap();

            assert_eq!(captures[1].trim(), "start");
            assert_eq!(captures[2].trim(), "mov $st, $vi(19)");
        }
    }

    #[test]
    fn label_regex() {
        let input = ".start";

        if !LABEL_RE.is_match(input) {
            panic!("input doesn't match a label");
        } else {
            let captures = LABEL_RE.captures_iter(input).next().unwrap();

            assert_eq!(captures[1].trim(), "start");
        }
    }
}
