//! Reads `method-table.toml`: a small subset of TOML, parsed here so the crate needs no dependency.
//!
//! The subset is arrays of tables (`[[method]]`), `key = value` lines, comments, and five kinds of
//! value: a basic string without escapes other than `\"` and `\\`, a multi-line literal string whose
//! closing `'''` sits on a line of its own, an integer (decimal, or hexadecimal with `0x` and
//! underscores), a boolean, and arrays of those or of inline tables. Anything else is refused with
//! its line number, so the file stays one that any TOML reader also reads.

/// One value of the subset.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Text(String),
    Number(u64),
    Flag(bool),
    List(Vec<Value>),
    Table(Vec<(String, Value)>),
}

/// One `[[kind]]` entry, its keys in file order.
pub struct Entry {
    pub kind: String,
    pub line: usize,
    pub keys: Vec<(String, Value)>,
}

struct Cursor<'a> {
    text: &'a [u8],
    at: usize,
}

impl Cursor<'_> {
    fn line(&self) -> usize {
        1 + self.text[..self.at].iter().filter(|&&b| b == b'\n').count()
    }

    fn fail<T>(&self, what: &str) -> Result<T, String> {
        Err(format!("method-table.toml:{}: {what}", self.line()))
    }

    fn peek(&self) -> Option<u8> {
        self.text.get(self.at).copied()
    }

    fn has_prefix(&self, prefix: &str) -> bool {
        self.text[self.at..].starts_with(prefix.as_bytes())
    }

    /// Spaces and tabs only: a key, its `=` and its value share a line.
    fn spaces(&mut self) {
        while matches!(self.peek(), Some(b' ' | b'\t')) {
            self.at += 1;
        }
    }

    /// Spaces, newlines and comments, as allowed between the items of an array.
    fn blank(&mut self) {
        loop {
            match self.peek() {
                Some(b' ' | b'\t' | b'\n') => self.at += 1,
                Some(b'#') => self.skip_line(),
                _ => return,
            }
        }
    }

    fn skip_line(&mut self) {
        while let Some(b) = self.peek() {
            self.at += 1;
            if b == b'\n' {
                return;
            }
        }
    }

    /// The end of a `key = value` line: optional spaces and comment, then a newline or the end.
    fn end_of_line(&mut self) -> Result<(), String> {
        self.spaces();
        match self.peek() {
            None => Ok(()),
            Some(b'\n' | b'#') => {
                self.skip_line();
                Ok(())
            }
            Some(_) => self.fail("text after the value"),
        }
    }

    fn word(&mut self) -> Result<String, String> {
        let start = self.at;
        while matches!(self.peek(), Some(b'a'..=b'z' | b'_' | b'0'..=b'9')) {
            self.at += 1;
        }
        if start == self.at {
            return self.fail("expected a key");
        }
        Ok(String::from_utf8(self.text[start..self.at].to_vec()).unwrap())
    }

    fn key_and_value(&mut self) -> Result<(String, Value), String> {
        let key = self.word()?;
        self.spaces();
        if self.peek() != Some(b'=') {
            return self.fail("expected `=` after the key");
        }
        self.at += 1;
        self.spaces();
        Ok((key, self.value()?))
    }

    fn value(&mut self) -> Result<Value, String> {
        if self.has_prefix("'''") {
            return self.literal_lines();
        }
        match self.peek() {
            Some(b'"') => self.text_value(),
            Some(b'[') => self.list(),
            Some(b'{') => self.inline_table(),
            Some(b'0'..=b'9') => self.number(),
            _ if self.has_prefix("true") => {
                self.at += 4;
                Ok(Value::Flag(true))
            }
            _ if self.has_prefix("false") => {
                self.at += 5;
                Ok(Value::Flag(false))
            }
            _ => self.fail("expected a value"),
        }
    }

    fn text_value(&mut self) -> Result<Value, String> {
        self.at += 1;
        let mut out = Vec::new();
        loop {
            match self.peek() {
                None | Some(b'\n') => return self.fail("unterminated string"),
                Some(b'"') => break,
                Some(b'\\') => {
                    self.at += 1;
                    match self.peek() {
                        Some(b @ (b'"' | b'\\')) => out.push(b),
                        _ => return self.fail("an escape other than \\\" or \\\\"),
                    }
                }
                Some(b) => out.push(b),
            }
            self.at += 1;
        }
        self.at += 1;
        Ok(Value::Text(String::from_utf8(out).unwrap()))
    }

    /// `'''`, a newline, then lines up to a line that is exactly `'''`. The value keeps every line
    /// with its newline, as TOML reads it.
    fn literal_lines(&mut self) -> Result<Value, String> {
        self.at += 3;
        if self.peek() != Some(b'\n') {
            return self.fail("a multi-line string starts on the line after its `'''`");
        }
        self.at += 1;
        let start = self.at;
        loop {
            if self.peek().is_none() {
                return self.fail("unterminated multi-line string");
            }
            let line_start = self.at;
            self.skip_line();
            let line = &self.text[line_start..self.at];
            if line == b"'''\n" || line == b"'''" {
                let body = &self.text[start..line_start];
                return Ok(Value::Text(String::from_utf8(body.to_vec()).unwrap()));
            }
            if line.windows(3).any(|w| w == b"'''") {
                return self.fail("`'''` inside a multi-line string");
            }
        }
    }

    fn number(&mut self) -> Result<Value, String> {
        let start = self.at;
        while matches!(
            self.peek(),
            Some(b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F' | b'x' | b'_')
        ) {
            self.at += 1;
        }
        let digits: String = std::str::from_utf8(&self.text[start..self.at])
            .unwrap()
            .chars()
            .filter(|&c| c != '_')
            .collect();
        let parsed = match digits.strip_prefix("0x") {
            Some(hex) => u64::from_str_radix(hex, 16),
            None => digits.parse(),
        };
        match parsed {
            Ok(n) => Ok(Value::Number(n)),
            Err(_) => self.fail("not a number"),
        }
    }

    fn list(&mut self) -> Result<Value, String> {
        self.at += 1;
        let mut items = Vec::new();
        loop {
            self.blank();
            if self.peek() == Some(b']') {
                self.at += 1;
                return Ok(Value::List(items));
            }
            items.push(self.value()?);
            self.blank();
            match self.peek() {
                Some(b',') => self.at += 1,
                Some(b']') => {}
                _ => return self.fail("expected `,` or `]` in an array"),
            }
        }
    }

    fn inline_table(&mut self) -> Result<Value, String> {
        self.at += 1;
        let mut keys = Vec::new();
        loop {
            self.spaces();
            if self.peek() == Some(b'}') {
                self.at += 1;
                return Ok(Value::Table(keys));
            }
            keys.push(self.key_and_value()?);
            self.spaces();
            match self.peek() {
                Some(b',') => self.at += 1,
                Some(b'}') => {}
                _ => return self.fail("expected `,` or `}` in an inline table"),
            }
        }
    }
}

/// Every `[[kind]]` entry of `text`, in file order. A key outside an entry is refused.
pub fn parse(text: &str) -> Result<Vec<Entry>, String> {
    let mut cursor = Cursor {
        text: text.as_bytes(),
        at: 0,
    };
    let mut entries: Vec<Entry> = Vec::new();
    loop {
        cursor.blank();
        if cursor.peek().is_none() {
            return Ok(entries);
        }
        if cursor.has_prefix("[[") {
            cursor.at += 2;
            let line = cursor.line();
            let kind = cursor.word()?;
            if !cursor.has_prefix("]]") {
                return cursor.fail("expected `]]`");
            }
            cursor.at += 2;
            cursor.end_of_line()?;
            entries.push(Entry {
                kind,
                line,
                keys: Vec::new(),
            });
            continue;
        }
        let pair = cursor.key_and_value()?;
        cursor.end_of_line()?;
        match entries.last_mut() {
            Some(entry) if entry.keys.iter().all(|(k, _)| *k != pair.0) => entry.keys.push(pair),
            Some(_) => return cursor.fail("a key given twice"),
            None => return cursor.fail("a key before the first `[[...]]`"),
        }
    }
}
