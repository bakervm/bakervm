//! Interrupts for communicating with the VM from the outside and also for
//! letting the VM communicate with the outside

use std::fmt::{self, Display, Formatter};
use std::str::FromStr;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Signal {
    FlushFrame,
}

impl Display for Signal {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            &Signal::FlushFrame => write!(f, "%flush_frame%"),
        }
    }
}

impl FromStr for Signal {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "%flush_frame%" => Ok(Signal::FlushFrame),
            _ => Err("unable to parse signal"),
        }
    }
}
