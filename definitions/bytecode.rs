use typedef::*;

pub const PREAMBLE: &str = "BAKER1";

/// Stops the program and shuts down the vm
pub const HALT: Byte = 0x01;

/// Adds the two top most values and pushes the result back to the data stack.
/// This also removes the processed values from the data stack.
pub const ADD: Byte = 0x02;

/// Subtracts the first value on the data stack from the second and pushes the
/// result back to the data stack. This also removes the processed values from
/// the data stack.
pub const SUB: Byte = 0x03;

/// Adds the two top most values and pushes the result back to the data stack.
/// This also removes the processed values from the data stack.
pub const MUL: Byte = 0x04;

/// Divides the second value on the data stack throuh the first one and pushes
/// the result back to the top of the data stack. This also removes the
/// processed values from the data stack.
pub const DIV: Byte = 0x05;

/// Prints out the top most value on the data stack
#[deprecated(since="0.1.0", note="will be replaced soon")]
pub const PRINT: Byte = 0x06;

/// Pushes a value (Word) to the data stack
pub const PUSH: Byte = 0x07;

/// An unconditional jump to the specified address
pub const JMP: Byte = 0x08;

/// Jumps to the specified address if the top most value on the data stack
/// equals 0
pub const JZ: Byte = 0x09;

/// Jumps to the specified address if the top most value on the data stack is
/// not equal to 0
pub const JNZ: Byte = 0x0A;

/// Moves the specified value (Word) to the specified register (Address)
pub const MOVW: Byte = 0x0B;

/// Moves the specified value (SmallWord) to the specified register (Address)
pub const MOVS: Byte = 0x0C;

/// Moves the specified value (TinyWord) to the specified register (Address)
pub const MOVT: Byte = 0x0D;

/// Moves the specified value (Byte) to the specified register (Address)
pub const MOVB: Byte = 0x0E;

/// Moves the value in the specified buffer register (Address) to the otherwise
/// specified buffer register (Address)
pub const MOV: Byte = 0x0F;

/// Moves the top most value of the data stack to the specified register
/// (Address)
pub const SMOV: Byte = 0x10;

/// Registers an interrupt callback by mapping the specified signal_id to the
/// given function call address
pub const REGI: Byte = 0x11;

/// Calls a function while pushing the return address to the return stack
pub const CALL: Byte = 0x12;

/// Returns from a function. The return value is pushed to the data stack
pub const RET: Byte = 0x13;
