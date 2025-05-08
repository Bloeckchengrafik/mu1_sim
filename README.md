# MU1 Simulator

The MU1 Simulator is a simple virtual machine that simulates a custom assembly-like instruction set. It allows you to write, assemble, and execute programs written in the MU1 assembly language. The simulator supports features like labels, conditions, memory monitoring, and self-modifying code.

This README provides an overview of the simulator, its syntax, and how to use it.

---

## Table of Contents

1. [Features](#features)
2. [Getting Started](#getting-started)
3. [Syntax](#syntax)
4. [Instruction Set](#instruction-set)
5. [Conditions](#conditions)
6. [Memory Monitoring](#memory-monitoring)
7. [Examples](#examples)
8. [How It Works](#how-it-works)

---

## Features

- **Custom Assembly Language**: Write programs using a simple assembly-like syntax.
- **Label Resolution**: Use labels for memory addresses and jump instructions.
- **Conditions**: Define conditions to check memory values after program execution.
- **Memory Monitoring**: Monitor specific memory locations during execution.
- **Self-Modifying Code**: Modify instructions in memory during runtime.
- **Interactive Memory Query**: Query memory values in different formats (signed, unsigned, hex, binary, or as instructions).

---

## Getting Started

### Prerequisites

- Rust (for building and running the simulator)

### Building the Simulator

Clone the repository and navigate to the `mu1_sim` directory:

```bash
git clone <repository-url>
cd mu1_sim
```

Build the simulator:

```bash
cargo build --release
```

Run the simulator with a sample program:

```bash
./target/release/mu1_sim samples/pointer.s
```

---

## Syntax

### Labels

Labels are used to mark memory addresses. A label is defined by appending a colon (`:`) to a name:

```s
Loop:
    LDA Total
    ADD One
    STO Total
    JGE Loop
```

### Instructions

Each line contains one instruction. Instructions can reference labels or constants.

```s
LDA Total    ; Load the value at the address of `Total` into the accumulator
ADD One      ; Add the value at the address of `One` to the accumulator
STO Total    ; Store the accumulator value at the address of `Total`
```

### Conditions

Conditions are defined using the `!` prefix. They are evaluated after the program finishes execution.

```s
!eq Total 183  ; Check if the value at `Total` equals 183
```

### Memory Monitoring

Memory monitoring is defined using the `%` prefix. The simulator will print the value of the monitored memory location during execution.

```s
%Count  ; Monitor the value at the address of `Count`
```

---

## Instruction Set

The MU1 instruction set consists of the following instructions:

| Instruction | Description                                                                 |
|-------------|-----------------------------------------------------------------------------|
| `LDA X`     | Load the value at address `X` into the accumulator.                        |
| `STO X`     | Store the accumulator value at address `X`.                                |
| `ADD X`     | Add the value at address `X` to the accumulator.                           |
| `SUB X`     | Subtract the value at address `X` from the accumulator.                    |
| `JMP X`     | Jump to address `X`.                                                       |
| `JGE X`     | Jump to address `X` if the accumulator is non-negative.                    |
| `JNE X`     | Jump to address `X` if the accumulator is not zero.                        |
| `STP`       | Stop program execution.                                                    |
| `CALL X`    | Call a subroutine at address `X`. Push the return address onto the stack.  |
| `RETURN`    | Return from a subroutine. Pop the return address from the stack.           |
| `PUSH`      | Push the accumulator value onto the stack.                                 |
| `POP`       | Pop the top value from the stack into the accumulator.                     |
| `LDR X`     | Load the value at the address stored at address `X` into the accumulator.  |
| `STR X`     | Store the accumulator value at the address stored at address `X`.          |
| `MOVPC`     | Move the accumulator value into the program counter (PC).                  |
| `MOVSP`     | Move the accumulator value into the stack pointer (SP).                    |
| `DEFW X`    | Define a word in memory with the value `X`.                                |

---

## Conditions

Conditions are used to verify the state of memory after the program finishes execution. They are defined using the `!` prefix.

### Syntax

```s
!eq <Label> <Value>
```

- `!eq`: Checks if the value at the address of `<Label>` equals `<Value>`.

### Example

```s
!eq Total 183  ; Verify that the value at `Total` is 183 after execution.
```

---

## Memory Monitoring

Memory monitoring allows you to observe specific memory locations during program execution. Monitored memory locations are printed after each instruction.

### Syntax

```s
%<Label>
```

### Example

```s
%Count  ; Monitor the value at the address of `Count`.
```

---

## Examples

### Pointer Example

This program calculates the sum of an array of numbers using a pointer.

```s
!eq Total 183

Loop:
    LDR TablePtr
    ADD Total
    STO Total
    LDA TablePtr
    ADD One
    STO TablePtr
    LDA Count
    SUB One
    STO Count
    JGE Loop
    STP

Total:
    DEFW 0
One:
    DEFW 1
Count:
    DEFW 4
TablePtr:
    DEFW Table
Table:
    DEFW 39
    DEFW 25
    DEFW 4
    DEFW 98
    DEFW 17
```

### Self-Modifying Code Example

This program demonstrates self-modifying code by modifying the `ADD` instruction during execution.

```s
!eq Total 183
%Count

Loop:
    LDA Total1
Add_instr:
    ADD Table
    STO Total
    LDA Add_instr
    ADD One
    STO Add_instr
    LDA Count
    SUB One
    STO Count
    JGE Loop
    STP

Total:
    DEFW 0
One:
    DEFW 1
Count:
    DEFW 4
Table:
    DEFW 39
    DEFW 25
    DEFW 4
    DEFW 98
    DEFW 17
```

---

## How It Works

1. **Parsing**: The simulator parses the input assembly file, resolving labels and instructions.
2. **Compilation**: Instructions are compiled into a 16-bit binary format.
3. **Execution**: The program is executed in a loop, with the program counter (PC), accumulator (AC), and stack pointer (SP) managing execution flow.
4. **Conditions**: After execution, conditions are evaluated to verify memory state.
5. **Interactive Query**: The user can query memory values in different formats.

---

## Interactive Memory Query

After program execution, you can query memory values interactively. The following formats are supported:

- `i`: Signed integer
- `u`: Unsigned integer
- `x`: Hexadecimal
- `b`: Binary
- `n`: Instruction

### Example

```plaintext
Enter memory address and format:
Formats: i (signed), u (unsigned), x (hex), b (binary), n (instruction)
Example: '42 i' or '100 x'
Enter 'q' to quit
```

---

## License

This project is licensed under the MIT License. See the LICENSE file for details.

---

Enjoy using the MU1 Simulator! 🎉
