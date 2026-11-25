//! The table's entries as types: what each key means, and the keys each kind of entry allows.

use crate::table::{Entry, Value};

/// What an entry is: a system call, a method on an object, or a named value one of them takes or
/// answers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    SystemCall,
    Method,
    Value,
}

/// How a call ends when it succeeds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Returns {
    /// It returns its answer words, or a negative error in word 0.
    Answer,
    /// It never returns.
    Never,
    /// It returns only to say why it was refused.
    Refusal,
}

/// One argument word.
pub struct Argument {
    pub name: String,
    /// A Rust expression for the bits the word defines, read from the crate root or, for a bare
    /// name, from the entry's own object module.
    pub bits: String,
    /// `true` for a word that is 0 or 1 and is passed as a `bool`.
    pub boolean: bool,
}

/// One row of the table.
pub struct Row {
    pub kind: Kind,
    /// The module that numbers the method or value; empty for a system call.
    pub object: String,
    /// The constant's name without any `SYSTEM_CALL_` prefix.
    pub name: String,
    pub number: u64,
    pub blank_line_before: bool,
    pub reserved: bool,
    pub wrapper: bool,
    pub returns: Returns,
    pub rights: Vec<String>,
    pub refusals: Vec<String>,
    pub arguments: Vec<Argument>,
    pub answers: Vec<String>,
    pub doc: String,
}

impl Row {
    /// The constant's name as the crate spells it.
    pub fn constant(&self) -> String {
        match self.kind {
            Kind::SystemCall => format!("SYSTEM_CALL_{}", self.name),
            _ => self.name.clone(),
        }
    }

    /// The constant's path from the crate root.
    pub fn path(&self) -> String {
        match self.kind {
            Kind::SystemCall => self.constant(),
            _ => format!("{}::{}", self.object, self.name),
        }
    }

    /// Whether the generator writes a typed wrapper for this row.
    pub fn has_wrapper(&self) -> bool {
        self.kind != Kind::Value && self.wrapper && !self.reserved
    }
}

const ALLOWED: &[(&str, &[&str])] = &[
    (
        "system_call",
        &[
            "name",
            "number",
            "blank_line_before",
            "wrapper",
            "returns",
            "arguments",
            "answers",
            "doc",
        ],
    ),
    (
        "method",
        &[
            "object",
            "name",
            "number",
            "blank_line_before",
            "reserved",
            "returns",
            "rights",
            "refusals",
            "arguments",
            "answers",
            "doc",
        ],
    ),
    (
        "value",
        &["object", "name", "number", "blank_line_before", "doc"],
    ),
];

fn fail<T>(entry: &Entry, what: &str) -> Result<T, String> {
    Err(format!(
        "method-table.toml:{}: [[{}]] {what}",
        entry.line, entry.kind
    ))
}

fn get<'a>(entry: &'a Entry, key: &str) -> Option<&'a Value> {
    entry.keys.iter().find(|(k, _)| k == key).map(|(_, v)| v)
}

fn text(entry: &Entry, key: &str) -> Result<String, String> {
    match get(entry, key) {
        Some(Value::Text(s)) => Ok(s.clone()),
        Some(_) => fail(entry, &format!("`{key}` is not a string")),
        None => fail(entry, &format!("has no `{key}`")),
    }
}

fn flag(entry: &Entry, key: &str, default: bool) -> Result<bool, String> {
    match get(entry, key) {
        Some(Value::Flag(b)) => Ok(*b),
        Some(_) => fail(entry, &format!("`{key}` is not a boolean")),
        None => Ok(default),
    }
}

fn texts(entry: &Entry, key: &str) -> Result<Vec<String>, String> {
    let Some(Value::List(items)) = get(entry, key) else {
        return fail(entry, &format!("has no array `{key}`"));
    };
    items
        .iter()
        .map(|item| match item {
            Value::Text(s) => Ok(s.clone()),
            _ => fail(
                entry,
                &format!("`{key}` holds a value that is not a string"),
            ),
        })
        .collect()
}

fn argument(entry: &Entry, item: &Value) -> Result<Argument, String> {
    let Value::Table(keys) = item else {
        return fail(entry, "has an argument that is not an inline table");
    };
    let mut argument = Argument {
        name: String::new(),
        bits: String::new(),
        boolean: false,
    };
    for (key, value) in keys {
        match (key.as_str(), value) {
            ("name", Value::Text(s)) => argument.name = s.clone(),
            ("bits", Value::Text(s)) => argument.bits = s.clone(),
            ("type", Value::Text(s)) if s == "bool" => argument.boolean = true,
            _ => {
                return fail(
                    entry,
                    &format!("has an argument key `{key}` it does not know"),
                );
            }
        }
    }
    if argument.name.is_empty() || argument.bits.is_empty() {
        return fail(entry, "has an argument without a `name` or `bits`");
    }
    Ok(argument)
}

fn arguments(entry: &Entry) -> Result<Vec<Argument>, String> {
    let Some(Value::List(items)) = get(entry, "arguments") else {
        return fail(entry, "has no array `arguments`");
    };
    items.iter().map(|item| argument(entry, item)).collect()
}

fn returns(entry: &Entry) -> Result<Returns, String> {
    match get(entry, "returns") {
        None => Ok(Returns::Answer),
        Some(Value::Text(s)) if s == "never" => Ok(Returns::Never),
        Some(Value::Text(s)) if s == "refusal" => Ok(Returns::Refusal),
        Some(_) => fail(entry, "`returns` is neither \"never\" nor \"refusal\""),
    }
}

/// The row an entry describes, with every key checked against its kind.
pub fn row(entry: &Entry) -> Result<Row, String> {
    let Some((_, allowed)) = ALLOWED.iter().find(|(k, _)| *k == entry.kind) else {
        return fail(entry, "is not a kind this table has");
    };
    if let Some((key, _)) = entry
        .keys
        .iter()
        .find(|(k, _)| !allowed.contains(&k.as_str()))
    {
        return fail(entry, &format!("has a key `{key}` its kind does not take"));
    }
    let kind = match entry.kind.as_str() {
        "system_call" => Kind::SystemCall,
        "method" => Kind::Method,
        _ => Kind::Value,
    };
    let Some(Value::Number(number)) = get(entry, "number") else {
        return fail(entry, "has no integer `number`");
    };
    let callable = kind != Kind::Value;
    Ok(Row {
        kind,
        object: if kind == Kind::SystemCall {
            String::new()
        } else {
            text(entry, "object")?
        },
        name: text(entry, "name")?,
        number: *number,
        blank_line_before: flag(entry, "blank_line_before", true)?,
        reserved: flag(entry, "reserved", false)?,
        wrapper: flag(entry, "wrapper", true)?,
        returns: returns(entry)?,
        rights: if kind == Kind::Method {
            texts(entry, "rights")?
        } else {
            Vec::new()
        },
        refusals: if kind == Kind::Method {
            texts(entry, "refusals")?
        } else {
            Vec::new()
        },
        arguments: if callable {
            arguments(entry)?
        } else {
            Vec::new()
        },
        answers: if callable {
            texts(entry, "answers")?
        } else {
            Vec::new()
        },
        doc: text(entry, "doc")?,
    })
}
