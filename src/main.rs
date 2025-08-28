mod asm;
mod compiler;
mod format;
mod lexer;

use clap::Parser;
use compiler::BFCompiler;
use format::format_code;
use lexer::BFLexer;
use std::process::Command;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Brainfuck source file
    filename: String,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Don't clean assembly file
    #[arg(short = 'a', long = "keep-asm")]
    keep_asm: bool,

    /// Only produce asm, don't assemble or link
    #[arg(short = 'A', long = "only-asm")]
    only_asm: bool,

    /// Output executable file name
    #[arg(short, long, default_value = "")]
    output: String,

    /// Tape size in bytes
    #[arg(short = 't', long = "tape-size", default_value_t = 30000)]
    tape_size: usize,

    /// Target architecture: unix, win64
    #[arg(short = 'p', long = "platform")]
    target_arch: Option<String>,

    /// Format Brainfuck source and exit
    #[arg(long = "format")]
    format: bool,
}

fn is_nasm_installed() -> bool {
    Command::new("nasm")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn is_ld_installed() -> bool {
    Command::new("ld")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn main() {
    let args = Args::parse();

    if args.format {
        let source = match std::fs::read_to_string(&args.filename) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to read source file: {}", e);
                std::process::exit(1);
            }
        };
        let formatted = format_code(&source);

        // write to file
        std::fs::write(&args.filename, formatted).expect("Failed to write formatted code to file");
        if args.verbose {
            println!("Formatted code written to {}", args.filename);
        }
        std::process::exit(0);
    }

    // Detect NASM and ld at start, but only if compilation will be needed
    if !args.only_asm {
        if !is_nasm_installed() {
            eprintln!("Error: NASM is not installed or not found in PATH.");
            std::process::exit(1);
        }
        if !is_ld_installed() {
            eprintln!("Error: ld is not installed or not found in PATH.");
            std::process::exit(1);
        }
    }

    // Auto-detect OS if target_arch not specified
    let detected_arch = if let Some(ref arch) = args.target_arch {
        arch.clone()
    } else {
        if cfg!(windows) {
            "win64".to_string()
        } else {
            "unix".to_string()
        }
    };
    let target_arch = detected_arch;

    let source = match std::fs::read_to_string(&args.filename) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to read source file: {}", e);
            std::process::exit(1);
        }
    };

    let mut lexer = BFLexer::new(source.chars());
    let tokens = lexer.tokenize();
    match BFLexer::<std::str::Chars>::check_syntax(&tokens) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Syntax error: {}", e);
            std::process::exit(1);
        }
    }
    let compiler = BFCompiler::new(tokens, args.tape_size, &target_arch, args.verbose);
    let asm = compiler.compile();
    let base = if args.output.is_empty() {
        format!("{}", &args.filename)
    } else {
        args.output.clone()
    };
    let nfile = format!("{}.temp.asm", base);
    let output_obj = format!("{}.temp.o", base);
    let output_exe = if args.output.is_empty() {
        format!("{}.temp.out", base)
    } else {
        base.clone()
    };
    std::fs::write(&nfile, asm).expect("Failed to write assembly file");
    if args.verbose {
        println!("Assembly code written to {}", nfile);
    }

    match target_arch.as_str() {
        "unix" => {
            if args.only_asm {
                if args.verbose {
                    println!(
                        "Only assembly output requested (-A). Skipping object and executable generation."
                    );
                }
            } else {
                if !is_nasm_installed() {
                    eprintln!("Error: NASM is not installed or not found in PATH.");
                    std::process::exit(1);
                }
                if !is_ld_installed() {
                    eprintln!("Error: ld is not installed or not found in PATH.");
                    std::process::exit(1);
                }
                let nasm_status = Command::new("nasm")
                    .args(&["-f", "elf64", &nfile, "-o", &output_obj])
                    .status()
                    .expect("Failed to execute NASM");
                if !nasm_status.success() {
                    eprintln!("Error: NASM failed to assemble the code.");
                    std::process::exit(1);
                }
                if args.verbose {
                    println!("Object file written to {}", output_obj);
                }

                let ld_status = Command::new("ld")
                    .args(&[&output_obj, "-o", &output_exe])
                    .status()
                    .expect("Failed to execute ld");
                if !ld_status.success() {
                    eprintln!("Error: ld failed to link the object file.");
                    std::process::exit(1);
                }
                if args.verbose {
                    println!("Executable file written to {}", output_exe);
                }
            }
        }
        "win64" => {
            if args.only_asm {
                if args.verbose {
                    println!(
                        "Only assembly output requested (-A). Skipping object and executable generation."
                    );
                }
            } else {
                if !is_nasm_installed() {
                    eprintln!("Error: NASM is not installed or not found in PATH.");
                    std::process::exit(1);
                }
                if !is_ld_installed() {
                    eprintln!("Error: ld is not installed or not found in PATH.");
                    std::process::exit(1);
                }
                let nasm_status = Command::new("nasm")
                    .args(&["-f", "win64", &nfile, "-o", &output_obj])
                    .status()
                    .expect("Failed to execute NASM");
                if !nasm_status.success() {
                    eprintln!("Error: NASM failed to assemble the code for win64.");
                    std::process::exit(1);
                }
                if args.verbose {
                    println!("Object file written to {}", output_obj);
                }

                // Link with ld for win64
                let ld_status = Command::new("ld")
                    .args(&[
                        &output_obj,
                        "-o",
                        &output_exe,
                        "-e",
                        "main",
                        "-subsystem",
                        "console",
                        "-lmsvcrt",
                    ])
                    .status()
                    .expect("Failed to execute ld");
                if !ld_status.success() {
                    eprintln!(
                        "Error: ld failed to link the object file for win64. Ensure ld is installed and in PATH."
                    );
                    std::process::exit(1);
                }
                if args.verbose {
                    println!("Executable file written to {} (ld)", output_exe);
                }
            }
        }
        _ => {
            eprintln!("Unknown target architecture: {:?}", args.target_arch);
            std::process::exit(1);
        }
    }

    if !args.keep_asm && !args.only_asm {
        std::fs::remove_file(&nfile).expect("Failed to remove temporary assembly file");
        std::fs::remove_file(&output_obj).expect("Failed to remove temporary object file");
        if args.verbose {
            println!("Temporary files removed.");
        }
    }
}
