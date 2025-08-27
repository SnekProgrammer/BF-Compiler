#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BFToken {
    IncrementPointer(u32),
    DecrementPointer(u32),
    IncrementValue(u32),
    DecrementValue(u32),
    OutputValue(u32),
    InputValue(u32),
    LoopStart,
    LoopEnd,
}

pub struct BFLexer<I>
where
    I: Iterator<Item = char>,
{
    pos: usize,
    input: std::iter::Peekable<I>,
}

impl<I> BFLexer<I>
where
    I: Iterator<Item = char>,
{
    pub fn new(input: I) -> Self {
        BFLexer { pos: 0, input: input.peekable() }
    }

    pub fn next_token(&mut self) -> Option<BFToken> {
        use BFToken::*;
        while let Some(&c) = self.input.peek() {
            match c {
                '>' | '<' | '+' | '-' | '.' | ',' => {
                    self.input.next();
                    self.pos += 1;
                    let mut count = 1;
                    // Combine repeated chars
                    while let Some(&next) = self.input.peek() {
                        if next == c {
                            self.input.next();
                            self.pos += 1;
                            count += 1;
                        } else {
                            break;
                        }
                    }
                    // Read number after char
                    let mut num_str = String::new();
                    while let Some(&next) = self.input.peek() {
                        if next.is_ascii_digit() {
                            num_str.push(next);
                            self.input.next();
                            self.pos += 1;
                        } else {
                            break;
                        }
                    }
                    let num = if !num_str.is_empty() {
                        match num_str.parse::<u32>() {
                            Ok(n) => n,
                            Err(e) => {
                                eprintln!("Error parsing number after '{}': {}", c, e);
                                count // fallback to count
                            }
                        }
                    } else {
                        count
                    };
                    return Some(match c {
                        '>' => IncrementPointer(num),
                        '<' => DecrementPointer(num),
                        '+' => IncrementValue(num),
                        '-' => DecrementValue(num),
                        '.' => OutputValue(num),
                        ',' => InputValue(num),
                        _ => unreachable!(),
                    });
                }
                '[' => {
                    self.input.next();
                    self.pos += 1;
                    return Some(LoopStart);
                },
                ']' => {
                    self.input.next();
                    self.pos += 1;
                    return Some(LoopEnd);
                },
                _ => {
                    self.input.next();
                    self.pos += 1;
                    continue;
                }, // skip non-command chars
            }
        }
        None
    }

    pub fn tokenize(&mut self) -> Vec<BFToken> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next_token() {
            tokens.push(token);
        }
        tokens
    }

    pub fn check_syntax(tokens: &[BFToken]) -> Result<(), String> {
        let mut stack = Vec::new();
        for (i, token) in tokens.iter().enumerate() {
            match token {
                BFToken::LoopStart => stack.push(i),
                BFToken::LoopEnd => {
                    if stack.pop().is_none() {
                        return Err(format!("Unmatched LoopEnd (]) at token {}", i));
                    }
                }
                _ => {}
            }
        }
        if let Some(pos) = stack.pop() {
            return Err(format!("Unmatched LoopStart ([) at token {}", pos));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lex(input: &str) -> Vec<BFToken> {
        let mut lexer = BFLexer::new(input.chars());
        lexer.tokenize()
    }

    #[test]
    fn test_combine_repeated_chars() {
        let tokens = lex("++++<<>>--");
        assert_eq!(tokens, vec![
            BFToken::IncrementValue(4),
            BFToken::DecrementPointer(2),
            BFToken::IncrementPointer(2),
            BFToken::DecrementValue(2),
        ]);
    }

    #[test]
    fn test_number_after_char() {
        let tokens = lex("+60 .2");
        assert_eq!(tokens, vec![
            BFToken::IncrementValue(60),
            BFToken::OutputValue(2),
        ]);
    }

    #[test]
    fn test_loops_and_ignore_non_commands() {
        let tokens = lex("[abc+2]--");
        assert_eq!(tokens, vec![
            BFToken::LoopStart,
            BFToken::IncrementValue(2),
            BFToken::LoopEnd,
            BFToken::DecrementValue(2),
        ]);
    }

    #[test]
    fn test_mixed() {
        let tokens = lex(">>+3[--.]<,1");
        assert_eq!(tokens, vec![
            BFToken::IncrementPointer(2),
            BFToken::IncrementValue(3),
            BFToken::LoopStart,
            BFToken::DecrementValue(2),
            BFToken::OutputValue(1),
            BFToken::LoopEnd,
            BFToken::DecrementPointer(1),
            BFToken::InputValue(1),
        ]);
    }

    #[test]
    fn test_check_syntax() {
        let tokens = lex("[--+.]");
        assert!(BFLexer::<std::str::Chars>::check_syntax(&tokens).is_ok());
        let tokens = lex("++[-->+++<]");
        assert!(BFLexer::<std::str::Chars>::check_syntax(&tokens).is_ok());
        let tokens = lex("[");
        assert!(BFLexer::<std::str::Chars>::check_syntax(&tokens).is_err());
        let tokens = lex("]");
        assert!(BFLexer::<std::str::Chars>::check_syntax(&tokens).is_err());
        let tokens = lex("[[[]]");
        assert!(BFLexer::<std::str::Chars>::check_syntax(&tokens).is_err());
    }
}