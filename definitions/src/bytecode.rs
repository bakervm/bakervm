use typedef::*;

/// Stops the program and shuts down the vm
pub const HALT: Byte = 0x01;

/// Adds the two top most values and pushes the result back to the stack. This also removes the
/// processed values from the stack.
pub const ADD: Byte = 0x02;

/// Subtracts the first value on the stack from the second and pushes the result back to the stack.
/// This also removes the processed values from the stack.
pub const SUB: Byte = 0x03;

/// Adds the two top most values and pushes the result back to the stack. This also removes the
/// processed values from the stack.
pub const MUL: Byte = 0x04;

/// Divides the second value on the stack throuh the first one and pushes the result back to the
/// top of the stack. This also removes the processed values from the stack.
pub const DIV: Byte = 0x05;

/// [DEPRECATED] Prints out the top most value on the stack
pub const PRINT: Byte = 0x06;

/// Pushes a value (Word) to the stack
pub const PUSH: Byte = 0x07;

/// An unconditional jump to the specified address
pub const JMP: Byte = 0x08;

/// Jumps to the specified address if the top most value on the stack equals 0
pub const JZ: Byte = 0x09;

/// Jumps to the specified address if the top most value on the stack is not equal to 0
pub const JNZ: Byte = 0x0A;

/// Moves the specified value (Word) to the specified register (Address)
pub const MOVW: Byte = 0x0B;

/// Moves the specified value (SmallWord) to the specified register (Address)
pub const MOVS: Byte = 0x0C;

/// Moves the specified value (TinyWord) to the specified register (Address)
pub const MOVT: Byte = 0x0D;

/// Moves the specified value (Byte) to the specified register (Address)
pub const MOVB: Byte = 0x0E;

/// Moves the value specified register (Address) to the otherwise specified register (Address)
pub const MOV: Byte = 0x0F;
