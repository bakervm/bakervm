use definitions::ImageBuilder;
use definitions::error::*;
use definitions::typedef::*;
use mnemonic::Mnemonic;
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

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
    last_base_dir: Option<PathBuf>,
    deep: usize,
}

impl BASMCompiler {
    fn add_label(&mut self, label: String) {
        self.label_addr_map.entry(label).or_insert(self.mnemonics.len());
    }

    fn base_path(&mut self, orig_path: &Path) -> Result<PathBuf> {
        let file_name = if let Some(ref file_name) = orig_path.file_name() {
            if let Some(..) = self.last_base_dir {
                PathBuf::from(orig_path)
            } else {
                PathBuf::from(file_name.clone())
            }
        } else {
            bail!("unable to obtain file name");
        };

        let absolute_path = if let Some(ref base_dir) = self.last_base_dir {
            base_dir.clone()
        } else {
            let current_dir = if orig_path.is_relative() {
                env::current_dir().chain_err(|| "unable to get current directory")?
            } else {
                if let Some(ref parent) = orig_path.parent() {
                    parent.to_path_buf().clone()
                } else {
                    bail!("unable to get parent directory")
                }
            };

            let base_file = current_dir.join(orig_path);
            let base_dir = if let Some(ref dir) = base_file.parent() {
                dir.clone()
            } else {
                bail!("unable to get parent directory")
            };

            self.last_base_dir = Some(base_dir.to_path_buf().clone());
            base_dir.to_path_buf()
        };

        Ok(absolute_path.join(file_name))
    }

    fn compile_mnemonics(&mut self, file_name: String) -> Result<()> {
        let orig_path = Path::new(&file_name);

        let padding = (0..self.deep).map(|_| "  ").collect::<String>();
        println!("BASM    {}{}", padding, file_name);

        let path = self.base_path(orig_path)?;

        let file = File::open(path).chain_err(|| "unable to open file")?;

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

                let first_half: String = if LABELED_MNEMONIC_RE.is_match(first_half) {
                    let captures = if let Some(captures) = LABELED_MNEMONIC_RE
                           .captures_iter(first_half)
                           .next() {
                        captures
                    } else {
                        bail!("no label capture found")
                    };

                    let label = captures[1].trim();

                    self.add_label(label.to_owned());

                    captures[2].trim().to_owned()
                } else if LABEL_RE.is_match(first_half) {
                    let captures =
                        if let Some(captures) = LABEL_RE.captures_iter(first_half).next() {
                            captures
                        } else {
                            bail!("no label capture found")
                        };

                    let label = captures[1].trim();

                    self.add_label(label.to_owned());
                    continue;
                } else if INCLUDE_RE.is_match(first_half) {
                    let captures =
                        if let Some(captures) = INCLUDE_RE.captures_iter(first_half).next() {
                            captures
                        } else {
                            bail!("no include capture found")
                        };

                    self.deep += 1;
                    self.compile_mnemonics(captures[1].trim().to_owned() + ".basm")?;
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

    fn compile_mnemonic(&mut self, mnemonic: Mnemonic) -> Result<()> {
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
            Mnemonic::Rev(vm_event, label) => {
                let addr = self.lookup(&label)?;
                self.builder.rev(vm_event, addr);
            }
            Mnemonic::Drop(target) => self.builder.drop(target),

            Mnemonic::Halt => self.builder.halt(),
            Mnemonic::Pause => self.builder.pause(),
            Mnemonic::Nop => self.builder.nop(),
            Mnemonic::Int(int) => self.builder.int(int),
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
        self.compile_mnemonics(file_name)?;

        for mnemonic in self.mnemonics.clone() {
            self.compile_mnemonic(mnemonic)?;
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
        "rev" => Ok(Mnemonic::Rev(args[0].parse()?, args[1].parse()?)),
        "drop" => Ok(Mnemonic::Drop(args[0].parse()?)),

        "halt" => Ok(Mnemonic::Halt),
        "pause" => Ok(Mnemonic::Pause),
        "nop" => Ok(Mnemonic::Nop),
        "int" => Ok(Mnemonic::Int(args[0].parse()?)),
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
