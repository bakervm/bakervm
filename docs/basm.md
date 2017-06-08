# BASM - The bakerVM assembly language

The bakerVM has a very simple architecture and ships its own very simple assembly language. The main part of the VM is the stack, where most of the computation should happen. Apart from that the bakerVM has a separate call-stack, so function calls won't pollute the data stack. In addition to that contains the bakerVM a so-called value-index, which is basically indexed RAM. It also has a framebuffer. The types and symbols used in the bakerVM assembly language are shown here.

## Target

| Mnemonic | Description                                                         |
|---------:|---------------------------------------------------------------------|
|    `$st` | Stack                                                               |
|    `$fb` | Framebuffer                                                         |
| `$vi(#)` | The value index, where the # represents a constant positive integer |

## Value

|                          Mnemonic | Description |
|----------------------------------:|-------------|
|                   `true`, `false` | Boolean     |
|           `1.3`, `4.7`, `-43.338` | Float       |
|            `1`, `2`, `43`, `-567` | Integer     |
|              `#23bb11`, `#774466` | Color       |
| `'a'`, `'b'`, `'/'`, `'\'`, `'@'` | Char        |

## Type

| Mnemonic | Description |
|---------:|-------------|
|   `bool` | Boolean     |
|  `float` | Float       |
|    `int` | Integer     |
|  `color` | Color       |
|   `char` | Char        |

## InternalInterrupt

| Mnemonic | Description      |
|---------:|------------------|
|      `0` | FlushFramebuffer |

## Label
A label is a marker in the source code that symbolizes an address in the instruction stream. Labels begin with a `.`, for example:
```
.start
  mov ....
  jmp ..
  push ..

.loop
  mov ....
  jmp ..
  push ..

.function_name
  mov ....
  jmp ..
  push ..
```
When called or jumped to, a label is only specified by its name:
```
jmp start

jmpeq loop

call function_name
```


## Instruction

|                 Mnemonic | Arguments                          | Description                                                                                              |
|-------------------------:|------------------------------------|----------------------------------------------------------------------------------------------------------|
|          `add dest, src` | dest: Target, src: Target          | Adds the value at the *src* target to the value at the *dest* target, consuming the *src* target         |
|          `sub dest, src` | dest: Target, src: Target          | Subtracts the value at the *src* target from the value at the *dest* target, consuming the *src* target  |
|          `div dest, src` | dest: Target, src: Target          | Divides the value at the *src* target through the value at the *dest* target, consuming the *src* target |
|          `mul dest, src` | dest: Target, src: Target          | Multiplies the value at the *src* target with the value at the *dest* target, consuming the *src* target |
|          `rem dest, src` | dest: Target, src: Target          | Calculates the remainder of the division *dest* / *src*, consuming the *src* target                      |
| `cmp target_a, target_b` | target_a: Target, target_b: Target | Compares the two targets saving the result (`Less`, `Greater`, `Equal`) to the compare register          |
|              `jmp label` | label: Label                       | Jumps unconditionally to the given label                                                                 |
|            `jmplt label` | label: Label                       | Jumps to the given label if the result of the last comparison is `Less`                                  |
|            `jmpgt label` | label: Label                       | Jumps to the given label if the result of the last comparison is `Greater`                               |
|            `jmpeq label` | label: Label                       | Jumps to the given label if the result of the last comparison is `Equal`                                 |
|          `jmplteq label` | label: Label                       | Jumps to the given label if the result of the last comparison is either `Less` or `Equal`                |
|          `jmpgteq label` | label: Label                       | Jumps to the given label if the result of the last comparison is either `Greater` or `Equal`             |
|      `cast target, type` | target: Target, type: Type         | Casts the value at the given target to the given type in-place                                           |
|     `push target, value` | target: Target, value: Value       | Pushes the given value to the given target                                                               |
|          `mov dest, src` | dest: Target, src: Target          | Moves the value at the *src* target to the *dest* target, consuming the *src* target                     |
| `swp target_a, target_b` | target_a: Target, target_b: Target | Swaps the values of the given targets                                                                    |
|             `dup target` | target: Target                     | Duplicates the value at the given target and pushes it to the stack                                      |
|             `call label` | label: Label                       | Calls the function at the given label, pushing the return address to the call-stack                      |
|                    `ret` | -                                  | Returns from a function call                                                                             |
|                   `halt` | -                                  | Halts the execution of the current program and causes the VM to shut down                                |
|                  `pause` | -                                  | Pauses the execution of the current program until an interrupt is received                               |
|                    `nop` | -                                  | Does nothing. Good for optimizing code                                                                   |
|          `int interrupt` | interrupt: InternalInterrupt       | Triggers the given internal interrupt                                                                    |
