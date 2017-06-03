pub enum Mnemonic {
    Add,
    Sub,
    Div,
    Mul,
    Rem,

    Cmp,
    Jmp,
    JmpLt,
    JmpGt,
    JmpEq,
    JmpLtEq,
    JmpGtEq,

    Cast,

    Push,
    Mov,
    Swp,

    Call,
    Ret,

    Halt,
    Pause,
    Int,
}

impl Mnemonic {
    pub fn from_str(string: &'static str) -> Mnemonic {
        match string {
            "add" => Mnemonic::Add,
            "sub" => Mnemonic::Sub,
            "div" => Mnemonic::Div,
            "mul" => Mnemonic::Mul,
            "rem" => Mnemonic::Rem,

            "cmp" => Mnemonic::Cmp,
            "jmp" => Mnemonic::Jmp,
            "jmplt" => Mnemonic::JmpLt,
            "jmpgt" => Mnemonic::JmpGt,
            "jmpeq" => Mnemonic::JmpEq,
            "jmplteq" => Mnemonic::JmpLtEq,
            "jmpgteq" => Mnemonic::JmpGtEq,

            "cast" => Mnemonic::Cast,

            "push" => Mnemonic::Push,
            "mov" => Mnemonic::Mov,
            "swp" => Mnemonic::Swp,

            "call" => Mnemonic::Call,
            "ret" => Mnemonic::Ret,

            "halt" => Mnemonic::Halt,
            "pause" => Mnemonic::Pause,
            "int" => Mnemonic::Int,
            _ => panic!("unknown mnemonic string"),
        }
    }
}
