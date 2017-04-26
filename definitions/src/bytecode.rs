use super::typedef::*;

/// Halts the program and shuts down the VM
/// `halt`
pub const HALT: Byte = 0x01;


/// Adds up the two top most values of the stack.
/// `add`
pub const ADD: Byte = 0x02;


/// Subtracts the two top most values of the stack.
/// `sub`
pub const SUB: Byte = 0x03;


/// Multiplies the two top most values of the stack.
/// `mul`
pub const MUL: Byte = 0x04;


/// Divides the two top most values of the stack.
///
/// # Examples
///
/// `div`
pub const DIV: Byte = 0x05;


/// Prints out the value on top of the stack.
///
/// # Examples
///
/// `print`
pub const PRINT: Byte = 0x06;


/// Pushes an unsigned integer to the stack. After push has been decoded, the next 4 bytes have to
/// be read onto the stack as one `u32`
///
/// # Examples
///
/// `push 18` => `07 00 00 00 12`
pub const PUSH: Byte = 0x07;


/// Jumps to the specified address in the code
///
/// # Examples
///
/// ```
/// push 1      ; 9 bytes
/// push 1      ; 9 bytes
/// add         ; 1 byte
/// print       ; 1 byte
/// jmp 9       ; 9 bytes (Jump to address 9)
/// ```
pub const JMP: Byte = 0x08;


/// Jumps to the specified address in the code if the value on top of the stack equals zero
///
/// # Examples
///
/// ```
/// push 0      ; 9 bytes
/// push 1      ; 9 bytes
/// add         ; 1 byte
/// print       ; 1 byte
/// jz 9       ; 9 bytes (Jump to address 9 if top of stack is zero)
/// ```
pub const JZ: Byte = 0x09;


/// Jumps to the specified address in the code if the value on top of the stack does not equal zero
///
/// # Examples
///
/// ```
/// push 101      ; 9 bytes
/// push 1      ; 9 bytes
/// sub         ; 1 byte
/// print       ; 1 byte
/// jnz 9       ; 9 bytes (Jump to address 9 if top of stack is not zero)
/// ```
pub const JNZ: Byte = 0x0A;
