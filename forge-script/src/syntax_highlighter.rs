use crate::script::ScriptType;

pub struct SyntaxHighlighter {
    pub current_language: ScriptType,
}

impl SyntaxHighlighter {
    pub fn new() -> Self {
        Self {
            current_language: ScriptType::GDScript,
        }
    }

    pub fn set_language(&mut self, language: ScriptType) {
        self.current_language = language;
    }

    pub fn highlight(&self, code: &str) -> Vec<(usize, usize, String)> {
        let mut result = Vec::new();
        let mut lines = code.lines();
        let mut line_num = 0;

        while let Some(line) = lines.next() {
            let mut chars = line.chars().peekable();
            let mut highlighted = String::new();
            let mut start = 0;

            while let Some(c) = chars.next() {
                if c == '\t' {
                    let tab_width = 4;
                    for _ in 0..tab_width {
                        highlighted.push(' ');
                    }
                    start += c.len_utf8();
                    continue;
                }

                if c == '#' {
                    let end = line[c.len_utf8()..].find('\n').unwrap_or(line.len() - c.len_utf8());
                    result.push((
                        line_num,
                        start,
                        format!("{}{}", " # ".repeat(1), " ".repeat(end)),
                    ));
                    start += c.len_utf8();
                    continue;
                }

                if c == '"' {
                    let end = line[c.len_utf8()..].find('"').unwrap_or(line.len() - c.len_utf8());
                    result.push((line_num, start, format!("\"{}\"", " ".repeat(end))));
                    start += c.len_utf8() + end;
                    continue;
                }

                if c == '(' || c == '{' || c == '[' {
                    highlighted.push(c);
                    start += c.len_utf8();
                    continue;
                }

                if c == ')' || c == '}' || c == ']' {
                    highlighted.push(c);
                    start += c.len_utf8();
                    continue;
                }

                if let Some(keyword) = self.find_keyword(&line[start..]) {
                    result.push((line_num, start, format!("{}", keyword)));
                    start += keyword.len();
                    continue;
                }

                highlighted.push(c);
                start += c.len_utf8();
            }

            line_num += 1;
        }

        result
    }

    fn find_keyword(&self, text: &str) -> Option<String> {
        let keywords = match self.current_language {
            ScriptType::GDScript => vec![
                "func", "var", "extends", "signal", "const", "class", "enum", "if", "else",
                "elif", "for", "while", "match", "return", "break", "continue", "pass",
                "await", "yield", "self", "super", "onready", "export", "tool", "preload",
                "setget", "func", "extends", "class_name", "enum", "signal", "const", "var",
            ],
            ScriptType::CSharp => vec![
                "class", "public", "private", "protected", "static", "void", "int", "string",
                "float", "double", "bool", "return", "if", "else", "for", "while", "switch",
                "case", "break", "continue", "new", "this", "base", "override", "virtual",
            ],
            _ => vec![],
        };

                for keyword in keywords {
            if text.starts_with(&keyword) {
                return Some(keyword.to_string());
            }
        }

        None
    }
}
