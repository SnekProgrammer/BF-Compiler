![Lines of code](https://img.shields.io/badge/lines-1237-blue?style=flat-square)

A [nice](http://www.muppetlabs.com/~breadbox/bf/standards.html) Brainfuck compiler written in Rust, translating Brainfuck source code into optimized x86-64 assembly for UNIX and Windows systems. Automatically assembles and links the generated assembly to produce a native executable, or outputs assembly code directly for manual use.

## Features
- Converts Brainfuck code to x86-64 assembly
- Supports custom tape size
- Combines repeated instructions for optimization
- Syntax checking for matching loops

## Requirements
- Rust (stable)
- NASM (Netwide Assembler) (For auto-assembly)
- ld (GNU linker) (For auto-assembly)

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
- Input and output are done using system calls on UNIX, and with the C runtime on windows

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
- `-p <arch>`, `--platform <arch>` : Target architecture (`unix` or `win64`)
- `--format` : Format Brainfuck source and print to stdout

### Example
```
./target/release/bf hello.b -o hello -v
./target/release/bf hello.b --format
```

## Testing
Run unit tests:
```
cargo test
```

## License
MIT
