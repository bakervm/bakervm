# The bakerVM instruction set
The bakerVM implements its very own highly performant instruction set. Every instruction has a designated functionality. The instructions are defined by the file `instruction.rs`:
```rust
pub enum Instruction {
    Add(Target, Target),
    Sub(Target, Target),
    Div(Target, Target),
    Mul(Target, Target),
    Rem(Target, Target),

    Cmp(Target, Target),
    Jmp(Address),
    JmpLt(Address),
    JmpGt(Address),
    JmpEq(Address),
    JmpLtEq(Address),
    JmpGtEq(Address),

    Cast(Target, Type),

    Push(Target, Value),
    Mov(Target, Target),
    Swp(Target, Target),
    Dup(Target),

    Call(Address),
    Ret,

    Halt,
    Pause,
    Nop,
    Sig(Signal),
}
```

## Instruction listing

|              Instruction | Mnemonic                             | State<br>[before] → [after]                                                            | Description                                                                                              |
|-------------------------:|--------------------------------------|----------------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------|
| Add(Target,&nbsp;Target) | add&nbsp;`dest`,&nbsp;`src`          | **dest**: `value`&nbsp;→&nbsp;`value`<br>**src**: `value`&nbsp;→                       | Adds the values of the `src` and `dest` targets                                                          |
| Sub(Target,&nbsp;Target) | sub&nbsp;`dest`,&nbsp;`src`          | **dest**: `value`&nbsp;→&nbsp;`value`<br>**src**: `value`&nbsp;→                       | Subtracts the value of the `src` target from the value of the `dest` target                              |
| Div(Target,&nbsp;Target) | div&nbsp;`dest`,&nbsp;`src`          | **dest**: `value`&nbsp;→&nbsp;`value`<br>**src**: `value`&nbsp;→                       | Divides the value of the `dest` target through the value of the `src` target                             |
| Mul(Target,&nbsp;Target) | mul&nbsp;`dest`,&nbsp;`src`          | **dest**: `value`&nbsp;→&nbsp;`value`<br>**src**: `value`&nbsp;→                       | Multiplies the values of the `src` and `dest` targets                                                    |
| Rem(Target,&nbsp;Target) | rem&nbsp;`dest`,&nbsp;`src`          | **dest**: `value`&nbsp;→&nbsp;`value`<br>**src**: `value`&nbsp;→                       | Calulates the remainder of the division `dest`/`src`                                                     |
| Cmp(Target,&nbsp;Target) | cmp&nbsp;`target_a`,&nbsp;`target_b` | **cmp_register**: `ordering`&nbsp;→&nbsp;`ordering`                                    | Compares the two targets saving the result into the `cmp_register`                                       |
|             Jmp(Address) | jmp&nbsp;`jump_target`               | [no&nbsp;change]                                                                       | Jumps unconditionally to the specified `jump_target`                                                     |
|           JmpLt(Address) | jmplt&nbsp;`jump_target`             | [no&nbsp;change]                                                                       | Jumps to the specified `jump_target` if the result of the last comparison is `less`                      |
|           JmpGt(Address) | jmpgt&nbsp;`jump_target`             | [no&nbsp;change]                                                                       | Jumps to the specified `jump_target` if the result of the last comparison is `greater`                   |
|           JmpEq(Address) | jmpeq&nbsp;`jump_target`             | [no&nbsp;change]                                                                       | Jumps to the specified `jump_target` if the result of the last comparison is `equal`                     |
|         JmpLtEq(Address) | jmplteq&nbsp;`jump_target`           | [no&nbsp;change]                                                                       | Jumps to the specified `jump_target` if the result of the last comparison is either `less` or `equal`    |
|         JmpGtEq(Address) | jmpgteq&nbsp;`jump_target`           | [no&nbsp;change]                                                                       | Jumps to the specified `jump_target` if the result of the last comparison is either `greater` or `equal` |
|       Cast(Target, Type) | cast&nbsp;`target`, `type`           | **target**: `value`&nbsp;→&nbsp;`value`                                                | Casts the value of the `target` into the specified `type` in-place                                       |
|      Push(Target, Value) | push&nbsp;`target`,&nbsp;`value`     | **target**: →&nbsp;`value`                                                             | Writes the `value` to the target                                                                         |
|      Mov(Target, Target) | mov&nbsp;`dest`,&nbsp;`src`          | **dest**: →&nbsp;`value`<br>**src**: `value`&nbsp;→                                    | Moves the value of the `src` target to the `dest` target                                                 |
|      Swp(Target, Target) | swp&nbsp;`target_a`,&nbsp;`target_b` | **target_a**: `value`&nbsp;→&nbsp;`value`<br>**target_b**: `value`&nbsp;→&nbsp;`value` | Swaps the values of `target_a` and `target_b`                                                            |
|              Dup(Target) | dup&nbsp;`target`                    | **stack**: →&nbsp;`value`                                                              | Duplicates the value of `target` to the `stack`                                                          |
|            Call(Address) | call&nbsp;`call_target`              | **call_stack**: →&nbsp;`address`                                                       | Calls the `call_target` pushing the return address to the `call_stack`                                   |
|                      Ret | ret                                  | **call_stack**: `address`&nbsp;→                                                       | Returns from a call using the top most address on the `call_stack`                                       |
|                     Halt | halt                                 | [no&nbsp;change]                                                                       | Halts the execution of the current program and causes the VM to shut down                                |
|                    Pause | pause                                | [no&nbsp;change]                                                                       | Pauses the execution of the current program until an event is received                                   |
|                      Nop | nop                                  | [no&nbsp;change]                                                                       | Does nothing. Good for optimizing code                                                                   |
|              Sig(Signal) | sig&nbsp;`signal`                    | [no&nbsp;change]                                                                       | Triggers the given `signal`                                                                              |
