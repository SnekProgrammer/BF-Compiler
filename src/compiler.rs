use crate::asm::Assembler;
use crate::lexer::BFToken;

pub struct BFCompiler {
    pub tokens: Vec<BFToken>,
    pub tape_size: usize,
    pub target_arch: String,
    pub pretty: bool,
}

impl BFCompiler {
    pub fn new(tokens: Vec<BFToken>, tape_size: usize, target_arch: &str, pretty: bool) -> Self {
        BFCompiler {
            tokens,
            tape_size,
            target_arch: target_arch.to_string(),
            pretty,
        }
    }
    pub fn compile(&self) -> String {
        let mut assembler = Assembler::new(&self.target_arch, self.pretty, self.tape_size).header();
        let mut loop_stack = Vec::new();
        let mut loop_id = 0;
        for token in &self.tokens {
            match token {
                BFToken::IncrementPointer(n) => {
                    assembler = assembler.inc_pointer(*n);
                }
                BFToken::DecrementPointer(n) => {
                    assembler = assembler.dec_pointer(*n);
                }
                BFToken::IncrementValue(n) => {
                    assembler = assembler.inc_value(*n);
                }
                BFToken::DecrementValue(n) => {
                    assembler = assembler.dec_value(*n);
                }
                BFToken::OutputValue(n) => {
                    assembler = assembler.output_value(*n);
                }
                BFToken::InputValue(n) => {
                    assembler = assembler.input_value(*n);
                }
                BFToken::LoopStart => {
                    assembler = assembler.loop_start(loop_id);
                    loop_stack.push(loop_id);
                    loop_id += 1;
                }
                BFToken::LoopEnd => {
                    if let Some(id) = loop_stack.pop() {
                        assembler = assembler.loop_end(id);
                    }
                }
            }
        }
        assembler = assembler.footer();
        assembler.build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::BFToken;

    #[test]
    fn test_increment_pointer() {
        let compiler = BFCompiler::new(vec![BFToken::IncrementPointer(3)], 90000, "unix", true);
        let asm = compiler.compile();
        assert!(asm.contains("add rsi, 3"));
    }

    #[test]
    fn test_decrement_value_and_output() {
        let compiler = BFCompiler::new(
            vec![BFToken::DecrementValue(2), BFToken::OutputValue(1)],
            90000,
            "unix",
            true,
        );
        let asm = compiler.compile();
        assert!(asm.contains("sub byte [rsi], 2"));
        assert!(asm.contains("sys_write"));
    }

    #[test]
    fn test_loop_generation() {
        let compiler = BFCompiler::new(
            vec![
                BFToken::LoopStart,
                BFToken::IncrementValue(1),
                BFToken::LoopEnd,
            ],
            90000,
            "unix",
            true,
        );
        let asm = compiler.compile();
        assert!(asm.contains("loop_start_0:"));
        assert!(asm.contains("loop_end_0:"));
        assert!(asm.contains("add byte [rsi], 1"));
    }

    #[test]
    fn test_input_and_exit() {
        let compiler = BFCompiler::new(vec![BFToken::InputValue(1)], 90000, "unix", true);
        let asm = compiler.compile();
        assert!(asm.contains("sys_read"));
        assert!(asm.contains("sys_exit"));
    }
}
