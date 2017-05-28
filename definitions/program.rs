use PREAMBLE;
use config::VMConfig;
use instruction::Instruction;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Program {
    pub preamble: String,
    pub version: String,
    pub config: VMConfig,
    pub instructions: Vec<Instruction>,
}

impl Default for Program {
    fn default() -> Self {
        Program {
            preamble: String::from(PREAMBLE),
            version: String::from(env!("CARGO_PKG_VERSION")),
            config: Default::default(),
            instructions: Default::default(),
        }
    }
}
