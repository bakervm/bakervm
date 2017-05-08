mod image;
mod stack;
mod call_stack;
mod interrupt;

use self::call_stack::{Call, CallStack};
use self::image::Image;
use self::interrupt::Interrupt;
use self::stack::Stack;
use definitions::bytecode;
use definitions::typedef::*;
use error::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{self, MapAccess, SeqAccess, Visitor};
use serde::ser::SerializeStruct;
use std::collections::HashMap;
use std::fmt;
use std::path::Path;
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};

/// The whole state of the VM
#[derive(Debug)]
pub struct VM {
    image: Image,
    data_stack: Stack,
    call_stack: CallStack,
    inter_reg: HashMap<Word, Address>,
    pub interrupt_sender: Sender<Interrupt>,
    interrupt_receiver: Receiver<Interrupt>,
}

impl Serialize for VM {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut vm = serializer.serialize_struct("vm", 4)?;
        vm.serialize_field("image", &self.image)?;
        vm.serialize_field("data_stack", &self.data_stack)?;
        vm.serialize_field("call_stack", &self.call_stack)?;
        vm.serialize_field("inter_reg", &self.inter_reg)?;
        vm.end()
    }
}

impl<'de> Deserialize<'de> for VM {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Image,
            DataStack,
            CallStack,
            InterReg,
        }

        struct VMVisitor;

        impl<'de> Visitor<'de> for VMVisitor {
            type Value = VM;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct VM")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<VM, V::Error>
                where V: SeqAccess<'de>
            {
                let image = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let data_stack = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let call_stack = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                let inter_reg = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(3, &self))?;

                let base_vm = VM::new();

                Ok(
                    VM {
                        image,
                        data_stack,
                        call_stack,
                        inter_reg,
                        ..base_vm
                    },
                )
            }

            fn visit_map<V>(self, mut map: V) -> Result<VM, V::Error>
                where V: MapAccess<'de>
            {
                let mut image = None;
                let mut data_stack = None;
                let mut call_stack = None;
                let mut inter_reg = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Image => {
                            if image.is_some() {
                                return Err(de::Error::duplicate_field("image"));
                            }
                            image = Some(map.next_value()?);
                        }
                        Field::DataStack => {
                            if data_stack.is_some() {
                                return Err(de::Error::duplicate_field("data_stack"));
                            }
                            data_stack = Some(map.next_value()?);
                        }
                        Field::CallStack => {
                            if call_stack.is_some() {
                                return Err(de::Error::duplicate_field("call_stack"));
                            }
                            call_stack = Some(map.next_value()?);
                        }
                        Field::InterReg => {
                            if inter_reg.is_some() {
                                return Err(de::Error::duplicate_field("inter_reg"));
                            }
                            inter_reg = Some(map.next_value()?);
                        }
                    }
                }
                let image = image.ok_or_else(|| de::Error::missing_field("image"))?;
                let data_stack = data_stack.ok_or_else(|| de::Error::missing_field("data_stack"))?;
                let call_stack = call_stack.ok_or_else(|| de::Error::missing_field("call_stack"))?;
                let inter_reg = inter_reg.ok_or_else(|| de::Error::missing_field("inter_reg"))?;

                let base_vm = VM::new();

                Ok(
                    VM {
                        image,
                        data_stack,
                        call_stack,
                        inter_reg,
                        ..base_vm
                    },
                )
            }
        }

        const FIELDS: &'static [&'static str] = &["image", "data_stack", "inter_reg"];
        deserializer.deserialize_struct("vm", FIELDS, VMVisitor)
    }
}

impl VM {
    pub fn new() -> VM {
        let (sender, receiver) = mpsc::channel::<Interrupt>();

        VM {
            image: Image::default(),
            data_stack: Stack::default(),
            call_stack: CallStack::default(),
            inter_reg: HashMap::new(),
            interrupt_sender: sender,
            interrupt_receiver: receiver,
        }
    }

    pub fn exec<P: AsRef<Path>>(&mut self, path: P) -> VMResult<()> {
        self.image.load(path).chain_err(|| "unable to load image")?;

        self.image.check_preamble().chain_err(|| "malformed preamble")?;

        while self.image.pc < self.image.data.len() {
            let interrupted = self.handle_intertupts().chain_err(|| "unable to handle interrupts")?;

            if interrupted {
                continue;
            }

            let byte = self.image.current_byte().chain_err(|| "unable to read current byte")?;

            match byte {
                bytecode::HALT => break,
                bytecode::ADD => {
                    self.data_stack.add().chain_err(|| "unable to execute 'add' instruction")?
                }
                bytecode::SUB => {
                    self.data_stack.sub().chain_err(|| "unable to execute 'sub' instruction")?
                }
                bytecode::MUL => {
                    self.data_stack.mul().chain_err(|| "unable to execute 'mul' instruction")?
                }
                bytecode::DIV => {
                    self.data_stack.div().chain_err(|| "unable to execute 'div' instruction")?
                }
                bytecode::PUSH => {
                    let res: Word = self.image.read_next().chain_err(|| "unable to read word")?;

                    self.data_stack
                        .push_word(res)
                        .chain_err(|| "unable to push value to the data stack")?;
                }
                bytecode::JMP => {
                    let addr: Word = self.image.read_next().chain_err(|| "unable to read word")?;

                    self.image.jmp(addr as Address);
                    continue;
                }
                bytecode::JZ => {
                    let addr: Word = self.image.read_next().chain_err(|| "unable to read word")?;

                    let top_of_stack =
                        self.data_stack
                            .peek_number()
                            .chain_err(|| "unable to get current top of data stack")?;

                    if top_of_stack == 0.0 {
                        self.image.jmp(addr as Address);
                        continue;
                    }
                }
                bytecode::JNZ => {
                    let addr: Word = self.image.read_next().chain_err(|| "unable to read word")?;

                    let top_of_stack =
                        self.data_stack
                            .peek_number()
                            .chain_err(|| "unable to get current top of data stack")?;

                    if top_of_stack != 0.0 {
                        self.image.jmp(addr as Address);
                        continue;
                    }
                }
                bytecode::CALL => {
                    let addr: Address =
                        self.image.read_next().chain_err(|| "unable to read address")?;

                    self.call(addr).chain_err(|| "unable to call function")?;
                    continue;
                }
                bytecode::RET => self.ret().chain_err(|| "unable to return from function call")?,
                bytecode::YLD => self.yld().chain_err(|| "unable to yield value")?,
                _ => {
                    bail!(
                        "unexpected opcode {:02x} at address {:?}",
                        byte,
                        self.image.pc
                    )
                }
            }

            self.image.advance_pc();
        }

        Ok(())
    }

    fn handle_intertupts(&mut self) -> VMResult<bool> {
        if self.inter_reg.is_empty() {
            return Ok(false);
        }

        let interrupt_message = self.interrupt_receiver.try_recv();

        match interrupt_message {
            Ok(mut interrupt) => {
                let addr: Address = self.inter_reg[&interrupt.signal_id];

                self.data_stack.append(&mut interrupt.arguments);

                self.call(addr).chain_err(|| "unable to call interrupt-related function")?;

                return Ok(true);
            }
            Err(TryRecvError::Disconnected) => bail!("interrupt channel disconnected"),
            _ => {}
        }

        Ok(false)
    }

    fn call(&mut self, address: Address) -> VMResult<()> {
        let stack_len = self.data_stack.data.len();

        let ret_addr = self.image.pc + 1;

        let call = Call::new(ret_addr, stack_len);

        self.call_stack.push(call);

        self.image.jmp(address);

        Ok(())
    }

    fn ret(&mut self) -> VMResult<()> {
        if self.call_stack.is_empty() {
            bail!("unable to yield values from an empty call stack");
        }

        // We unwrap here because we already checked if the call stack is empty
        let mut call = self.call_stack.pop().unwrap();

        self.data_stack.truncate(call.gc);

        self.data_stack.append(&mut call.yield_stack);

        self.image.jmp(call.ret_addr);

        Ok(())
    }

    fn yld(&mut self) -> VMResult<()> {
        if self.call_stack.is_empty() {
            bail!("unable to yield values from an empty call stack");
        }

        // We unwrap here because we already checked if the call stack is empty
        let mut call = self.call_stack.pop().unwrap();

        let value =
            self.data_stack.pop_word().chain_err(|| "unable to pop value off the data stack")?;

        call.yield_stack.push(value);

        self.call_stack.push(call);

        Ok(())
    }
}
