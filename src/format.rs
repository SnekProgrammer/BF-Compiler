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
                            need_tab = false;
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
    use super::format_code;

    #[test]
    fn test_basic_formatting() {
        let src = ">[<+>[-]]";
        let expected = ">
[
	<
	+
	>[
		-
	]
]
";
        let result = format_code(src);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_empty_input() {
        let src = "";
        let expected = "";
        let result = format_code(src);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_nested_loops() {
        let src = "[[[]]]";
        let expected = "[
\t[
\t\t[
\t\t]
\t]
]";
        let result = format_code(src);
        assert_eq!(result, expected);
    }
}

