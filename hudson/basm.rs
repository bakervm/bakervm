use definitions::ImageBuilder;
use definitions::error::*;
use definitions::typedef::*;
use mnemonic::Mnemonic;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

lazy_static! {
    static ref LABELED_MNEMONIC_RE: Regex = Regex::new(r"^\.(.+?) +?(.+)$").unwrap();
    static ref LABEL_RE: Regex = Regex::new(r"^\.([^\s]+)$").unwrap();
    static ref INCLUDE_RE: Regex = Regex::new(r"^include! +([^\s]+)$").unwrap();
}

#[derive(Default)]
struct BASMCompiler {
    label_addr_map: HashMap<String, Address>,
    mnemonics: Vec<Mnemonic>,
    builder: ImageBuilder,
    compiled_files: HashSet<String>,
    deep: usize,
}

impl BASMCompiler {
    fn add_label(&mut self, label: String) -> Result<()> {
        if self.label_addr_map.contains_key(&label) {
            bail!("label {:?} already exists", label)
        } else {
            self.label_addr_map.entry(label).or_insert(self.mnemonics.len());

            Ok(())
        }
    }

    fn compile_mnemonics(&mut self, orig_path: &Path) -> Result<()> {
        let padding = (0..self.deep).map(|_| "  ").collect::<String>();
        println!("BASM    {}{:?}", padding, orig_path);

        let path_string = if let Some(string) = orig_path.to_str() {
            string.to_owned()
        } else {
            bail!("unable to convert path to string");
        };

        if self.compiled_files.contains(&path_string) {
            return Ok(());
        } else {
            self.compiled_files.insert(path_string);
        }

        let file = File::open(orig_path).chain_err(|| "unable to open file")?;

        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.chain_err(|| "unable to read line")?;

            if line.is_empty() {
                continue;
            }

            let mut r_split = line.rsplitn(2, ';');

            if r_split.clone().count() > 1 {
                r_split.next();
            }

            if let Some(first_half) = r_split.next() {
                let first_half = first_half.trim();

                if first_half.is_empty() {
                    continue;
                }

                let first_half: String = if LABELED_MNEMONIC_RE.is_match(first_half) {
                    let captures = if let Some(captures) = LABELED_MNEMONIC_RE
                           .captures_iter(first_half)
                           .next() {
                        captures
                    } else {
                        bail!("no label capture found")
                    };

                    let label = captures[1].trim();

                    self.add_label(label.to_owned())?;

                    captures[2].trim().to_owned()
                } else if LABEL_RE.is_match(first_half) {
                    let captures =
                        if let Some(captures) = LABEL_RE.captures_iter(first_half).next() {
                            captures
                        } else {
                            bail!("no label capture found")
                        };

                    let label = captures[1].trim();

                    self.add_label(label.to_owned())?;
                    continue;
                } else if INCLUDE_RE.is_match(first_half) {
                    let captures =
                        if let Some(captures) = INCLUDE_RE.captures_iter(first_half).next() {
                            captures
                        } else {
                            bail!("no include capture found")
                        };

                    self.deep += 1;

                    let parent = if let Some(ref parent) = orig_path.parent() {
                        parent.to_path_buf().clone()
                    } else {
                        bail!("unable to get parent directory")
                    };

                    env::set_current_dir(parent.clone())
                        .chain_err(|| "unable to switch directories")?;

                    let path = Path::new(&(captures[1].trim().to_owned() + ".basm"))
                        .canonicalize()
                        .chain_err(|| "unable to canonicalize path")?;

                    self.compile_mnemonics(&path)?;

                    env::set_current_dir(parent).chain_err(|| "unable to switch directories")?;
                    self.deep -= 1;

                    continue;
                } else {
                    first_half.to_owned()
                };

                let mut first_half_split = first_half.splitn(2, ' ');

                if let Some(opcode) = first_half_split.next() {
                    let opcode = opcode.trim().to_lowercase();


                    let args: Vec<String> = if let Some(args) = first_half_split.next() {
                        args.split(',').map(|arg| arg.trim().to_owned()).collect()
                    } else {
                        Vec::new()
                    };

                    self.mnemonics.push(text_to_mnemonic(opcode, args)?);
                } else {
                    bail!("opcode expected. Found {:?}", first_half_split);
                }
            } else {
                bail!("instruction expected. Found {:?}", line.to_string());
            }
        }

        Ok(())
    }

    fn compile_instruction(&mut self, mnemonic: Mnemonic) -> Result<()> {
        match mnemonic {
            Mnemonic::Add(dest, src) => self.builder.add(dest, src),
            Mnemonic::Sub(dest, src) => self.builder.sub(dest, src),
            Mnemonic::Div(dest, src) => self.builder.div(dest, src),
            Mnemonic::Mul(dest, src) => self.builder.mul(dest, src),
            Mnemonic::Rem(dest, src) => self.builder.rem(dest, src),

            Mnemonic::Cmp(target_a, target_b) => self.builder.cmp(target_a, target_b),
            Mnemonic::Jmp(label) => {
                let addr = self.lookup(&label)?;
                self.builder.jmp(addr);
            }
            Mnemonic::JmpLt(label) => {
                let addr = self.lookup(&label)?;
                self.builder.jmp_lt(addr);
            }
            Mnemonic::JmpGt(label) => {
                let addr = self.lookup(&label)?;
                self.builder.jmp_gt(addr);
            }
            Mnemonic::JmpEq(label) => {
                let addr = self.lookup(&label)?;
                self.builder.jmp_eq(addr);
            }
            Mnemonic::JmpLtEq(label) => {
                let addr = self.lookup(&label)?;
                self.builder.jmp_lt_eq(addr);
            }
            Mnemonic::JmpGtEq(label) => {
                let addr = self.lookup(&label)?;
                self.builder.jmp_gt_eq(addr);
            }

            Mnemonic::Cast(target, type_t) => self.builder.cast(target, type_t),

            Mnemonic::Push(target, value) => self.builder.push(target, value),
            Mnemonic::Mov(dest, src) => self.builder.mov(dest, src),
            Mnemonic::Swp(target_a, target_b) => self.builder.swp(target_a, target_b),
            Mnemonic::Dup(target) => self.builder.dup(target),

            Mnemonic::Call(label) => {
                let addr = self.lookup(&label)?;
                self.builder.call(addr);
            }
            Mnemonic::Ret => self.builder.ret(),

            Mnemonic::Halt => self.builder.halt(),
            Mnemonic::Pause => self.builder.pause(),
            Mnemonic::Nop => self.builder.nop(),
            Mnemonic::Sig(sig) => self.builder.sig(sig),
        }

        Ok(())
    }

    fn lookup(&mut self, input: &String) -> Result<Address> {
        if let Some(addr) = self.label_addr_map.get(input) {
            Ok(*addr)
        } else {
            bail!("label {:?} not found", input);
        }
    }

    pub fn compile(&mut self, file_name: String) -> Result<ImageData> {
        let path =
            Path::new(&file_name).canonicalize().chain_err(|| "unable to canonicalize path")?;

        let parent = if let Some(ref parent) = path.parent() {
            parent.to_path_buf().clone()
        } else {
            bail!("unable to get parent directory")
        };

        env::set_current_dir(parent.clone()).chain_err(|| "unable to switch directories")?;

        self.compile_mnemonics(&path)?;

        env::set_current_dir(parent).chain_err(|| "unable to switch directories")?;

        for mnemonic in self.mnemonics.clone() {
            self.compile_instruction(mnemonic)?;
        }

        Ok(self.builder.clone().gen())
    }
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
        "sig" => Ok(Mnemonic::Sig(args[0].parse()?)),
        _ => bail!("unkwnown opcode {:?}", opcode),
    }
}

pub fn compile(file_name: String) -> Result<ImageData> {
    BASMCompiler::default().compile(file_name)
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

    #[test]
    fn include_regex() {
        let input = "include! std";

        if !INCLUDE_RE.is_match(input) {
            panic!("input doesn't match an include statement");
        } else {
            let captures = INCLUDE_RE.captures_iter(input).next().unwrap();

            assert_eq!(captures[1].trim(), "std");
        }
    }
}
