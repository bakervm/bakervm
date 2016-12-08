#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Instruction {
    /// Halts the program and shuts down the VM
    HALT,
    /// Adds up the two top most values of the stack
    IADD,
    /// Subtracts the two top most values of the stack
    ISUB,
    /// Multiplies the two top most values of the stack
    IMUL,
    /// Divides the two top most values of the stack
    IDIV,
    /// Prints out the value on top of the stack
    PRINT,
    /// Pushes the top most value on the stack to the given coordinates on the display
    PSTD(u16, u16),
    /// Pushes an unsigned integer to the stack
    PUSH(u64),
    /// Sets the size of the display
    SDS(u16, u16),
}
