# Brainfuck x86-64 Compiler

This project is a Brainfuck compiler written in Rust. It translates Brainfuck source code into x86-64 assembly for UNIX systems, assembles and links it to produce a native executable.

## Features
- Converts Brainfuck code to x86-64 assembly
- Supports custom tape size
- Combines repeated Brainfuck instructions for optimization
- Syntax checking for matching loops
- Command-line interface with options for verbose output, keeping intermediate files, and custom output names
- Unit tests for lexer and compiler

## Requirements
- Rust (stable)
- NASM (Netwide Assembler)
- ld (GNU linker)


## Syntax
- `>` : Increment the data pointer
- `<` : Decrement the data pointer
- `+` : Increment the byte at the data pointer
- `-` : Decrement the byte at the data pointer
- `.` : Output the byte at the data pointer as a character
- `,` : Input a byte and store it at the data pointer
- `[` : Jump forward to the command after the matching `]` if the byte at the data pointer is zero
- `]` : Jump back to the command after the matching `[` if the byte at the data pointer is non-zero

A number following any of the commands excluding `[` and `]` indicates repetition of that command. For example, `+5` is equivalent to `+++++`.
`+0` will do nothing, however it will result in a 'useless' instruction in the assembly (`add byte [rsi], 0`).

A number that is not directly after a command is ignored. For example, `5+` is equivalent to `+`, and `+ 5` likewise.
The same follows for any non-command character, meaning `+72. Hello World!` is equivalent to `+72.`.


there is no support for `#` nor `!`.

## Bf Specifications
- Memory tape of 30,000 cells (default, configurable)
- Unbalanced loops are detected and reported as errors
- Out of bounds memory access is not checked (undefined behavior)
- EOF on ',' is no-change (https://brainfuck.org/epistle.html § 4)
- Empty loops are allowed and do nothing but are not optimized away
- Cells are 8-bit and wrap on overflow/underflow
- Pointer will NOT wrap on overflow/underflow (undefined behavior)
- Input and output are done using system calls (Unix x86-64), this may or may not be expanded in the future.


## Usage

### Build
```
cargo build --release
```

### Run
```
./target/release/bf <source.b> [options]
```

#### Options
- `-v`, `--verbose` : Verbose output
- `-a`, `--keep-asm` : Keep intermediate assembly and object files
- `-A`, `--only-asm` : Only produce assembly, do not assemble or link
- `-o <name>`, `--output <name>` : Output executable file name
- `-t <size>`, `--tape-size <size>` : Tape size in bytes (default: 30000)

### Example
```
./target/release/bf hello.b -o hello -v
```

## Project Structure
- `src/lexer.rs` : Tokenizes Brainfuck source code
- `src/compiler.rs` : Converts tokens to x86-64 assembly
- `src/main.rs` : CLI and orchestration

## Testing
Run unit tests:
```
cargo test
```

## License
MIT
