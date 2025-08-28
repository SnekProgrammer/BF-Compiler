pub fn format_code(source: &str) -> String {
    let mut tabs: i32 = 0;
    let mut formatted = String::new();
    let mut chars = source.chars().peekable();
    let mut last_type: Option<char> = None;
    let mut first_bracket = true;
    let bf_commands = ['>', '<', '+', '-', '.', ',', '[', ']'];
    let mut need_tab = false;
    while let Some(c) = chars.next() {
        if c == '\n' || c == '\r' || (!bf_commands.contains(&c) && !c.is_ascii_digit()) {
            continue; // filter out newlines, comments, and non-command chars
        }
        let is_command = bf_commands.contains(&c);
        if is_command {
            if let Some(last) = last_type {
                if last != c {
                    formatted.push('\n');
                    need_tab = true;
                }
            }
            match c {
                '[' => {
                    if first_bracket {
                        formatted.push('[');
                        formatted.push('\n');
                        tabs += 1;
                        first_bracket = false;
                        need_tab = true;
                    } else {
                        if need_tab {
                            for _ in 0..tabs {
                                formatted.push('\t');
                            }
                            //need_tab = false;
                        }
                        formatted.push('[');
                        formatted.push('\n');
                        tabs += 1;
                        need_tab = true;
                    }
                }
                ']' => {
                    tabs = tabs.saturating_sub(1);
                    formatted.push('\n');
                    for _ in 0..tabs {
                        formatted.push('\t');
                    }
                    formatted.push(']');
                    need_tab = false;
                }
                _ => {
                    if need_tab {
                        for _ in 0..tabs {
                            formatted.push('\t');
                        }
                        need_tab = false;
                    }
                    formatted.push(c);
                }
            }
            last_type = Some(c);
        } else if c.is_ascii_digit() {
            formatted.push(c);
        }
    }
    formatted
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_formatting() {
        let src = "[>+ -]\n<";
        let expected = "[\n\n\t>\n\t+\n\t-\n\n]\n<";
        let formatted = format_code(src);
        assert_eq!(formatted, expected);
    }

    #[test]
    fn test_nested_brackets() {
        let src = "[>[+]<]";
        let expected = "[\n\n\t>\n\t[\n\n\t\t+\n\n\t]\n\t<\n\n]";
        let formatted = format_code(src);
        assert_eq!(formatted, expected);
    }

    #[test]
    fn test_ignores_non_commands() {
        let src = "+abc-123";
        let expected = "+\n-123";
        let formatted = format_code(src);
        assert_eq!(formatted, expected);
    }

    #[test]
    fn test_tabs_and_newlines() {
        let src = "[+[->]<]";
        let expected = "[\n\n\t+\n\t[\n\n\t\t-\n\t\t>\n\n\t]\n\t<\n\n]";
        let formatted = format_code(src);
        assert_eq!(formatted, expected);
    }
}
