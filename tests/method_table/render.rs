//! Writes Rust from the table's rows: the constants of each object module, `src/system_call.rs`
//! (typed calls and defined-bits masks), its test, and the pinned lines.

use crate::model::{Kind, Returns, Row};
use std::fmt::Write as _;

/// The line that opens a generated part of a hand-written file.
pub const BEGIN: &str = "// Generated from method-table.toml by tests/method_table: edit the table, then run\n\
// `MOHO_ABI_REGENERATE=1 cargo test --test method_table`.\n";
/// The line that closes it.
pub const END: &str = "// End of the part generated from method-table.toml.\n";

fn doc_lines(out: &mut String, indent: &str, doc: &str) {
    for line in doc.lines() {
        if line.is_empty() {
            let _ = writeln!(out, "{indent}///");
        } else {
            let _ = writeln!(out, "{indent}/// {line}");
        }
    }
}

fn value(number: u64) -> String {
    if number == u64::MAX {
        String::from("u64::MAX")
    } else {
        number.to_string()
    }
}

/// The generated part of `src/<object>.rs`, or of `src/lib.rs` for an empty `object`, markers
/// included.
pub fn constants(rows: &[Row], object: &str) -> String {
    let mut out = String::from(BEGIN);
    let mut first = true;
    for row in rows.iter().filter(|r| r.object == object) {
        if row.blank_line_before && !first {
            out.push('\n');
        }
        first = false;
        doc_lines(&mut out, "", &row.doc);
        let _ = writeln!(
            out,
            "pub const {}: u64 = {};",
            row.constant(),
            value(row.number)
        );
    }
    out.push_str(END);
    out
}

/// The line `src/tests.rs` and `pinned-at-version-1.txt` hold for a row.
pub fn pinned_line(row: &Row) -> String {
    let number = if row.number == u64::MAX {
        String::from("0xFFFF_FFFF_FFFF_FFFF")
    } else {
        row.number.to_string()
    };
    format!("crate::{} => {number},", row.path())
}

/// `text` with the generated part between `BEGIN` and `END` replaced by `part`.
pub fn splice(file: &str, text: &str, part: &str) -> Result<String, String> {
    let start = text
        .find(BEGIN)
        .ok_or_else(|| format!("{file} has no line opening a generated part"))?;
    let end = text[start..]
        .find(END)
        .ok_or_else(|| format!("{file} has no line closing its generated part"))?
        + start
        + END.len();
    if text[end..].contains(BEGIN) {
        return Err(format!("{file} has a second generated part"));
    }
    Ok(format!("{}{part}{}", &text[..start], &text[end..]))
}

/// The function name of a row's typed call: the constant in lower case, with `yield_now` for
/// `yield`, a keyword.
pub fn function(row: &Row) -> String {
    match row.name.as_str() {
        "YIELD" => String::from("yield_now"),
        name => name.to_lowercase(),
    }
}

/// A mask term read from the crate root: `rights::ALL` stays, a bare `MAP_CODE` is the entry's own
/// module's, and a literal or a `u64::` constant is left alone.
fn bits(row: &Row, expression: &str) -> String {
    expression
        .split(" | ")
        .map(|term| {
            if term.starts_with(|c: char| c.is_ascii_digit()) || term.starts_with("u64::") {
                String::from(term)
            } else if term.contains("::") {
                format!("super::super::{term}")
            } else {
                format!("super::super::{}::{term}", row.object)
            }
        })
        .collect::<Vec<_>>()
        .join(" | ")
}

fn answer_type(row: &Row) -> String {
    match (row.returns, row.answers.len()) {
        (Returns::Never, _) => String::from("()"),
        (Returns::Refusal, _) => String::from("i64"),
        (Returns::Answer, 0) => String::from("Result<(), i64>"),
        (Returns::Answer, 1) => String::from("Result<u64, i64>"),
        (Returns::Answer, n) => format!("Result<[u64; {n}], i64>"),
    }
}

fn answer_body(row: &Row) -> String {
    let up = if row.kind == Kind::SystemCall {
        ""
    } else {
        "super::"
    };
    match (row.returns, row.answers.len()) {
        (Returns::Never, _) => String::from("|_| ()"),
        (Returns::Refusal, _) => String::from("|words| words[0] as i64"),
        (Returns::Answer, 0) => format!("|words| {up}refusal(words).map(|_| ())"),
        (Returns::Answer, 1) => format!("|words| {up}refusal(words).map(|words| words[0])"),
        (Returns::Answer, n) => {
            let words: Vec<String> = (0..n).map(|i| format!("words[{i}]")).collect();
            format!(
                "|words| {up}refusal(words).map(|words| [{}])",
                words.join(", ")
            )
        }
    }
}

fn link(row: &Row) -> String {
    format!("[`{}`](crate::{})", row.constant(), row.path())
}

fn call_doc(out: &mut String, indent: &str, row: &Row) {
    let mut text = format!("The call {} makes, and how its answer reads.\n", link(row));
    if !row.answers.is_empty() {
        let words: Vec<String> = row
            .answers
            .iter()
            .enumerate()
            .map(|(i, a)| format!("`{a}` (word {i})"))
            .collect();
        let _ = writeln!(text, "\nIt answers {}.", words.join(", "));
    }
    text.push_str(match row.returns {
        Returns::Never => "\nIt never returns, so its answer has nothing to read.\n",
        Returns::Refusal => "\nIt returns only when refused, and the answer is the refusal.\n",
        Returns::Answer => "\nA refusal is the negative word 0, an [`Error`](crate::Error) code.\n",
    });
    if !row.rights.is_empty() {
        let rights: Vec<String> = row
            .rights
            .iter()
            .map(|r| format!("[`{r}`](crate::rights::{r})"))
            .collect();
        let _ = writeln!(text, "\nThe rights it reads: {}.", rights.join(", "));
    }
    if !row.refusals.is_empty() {
        let refusals: Vec<String> = row
            .refusals
            .iter()
            .map(|r| format!("[`{r}`](crate::Error::{r})"))
            .collect();
        let _ = writeln!(text, "\nIts own refusals: {}.", refusals.join(", "));
    }
    doc_lines(out, indent, &text);
}

fn call(out: &mut String, indent: &str, row: &Row) {
    call_doc(out, indent, row);
    let mut parameters: Vec<String> = Vec::new();
    let mut words: Vec<String> = Vec::new();
    let number = if row.kind == Kind::SystemCall {
        format!("super::{}", row.constant())
    } else {
        parameters.push(String::from("capability: u64"));
        words.push(String::from("capability"));
        words.push(format!("super::super::{}", row.path()));
        String::from("super::super::SYSTEM_CALL_INVOKE")
    };
    for argument in &row.arguments {
        let kind = if argument.boolean { "bool" } else { "u64" };
        parameters.push(format!("{}: {kind}", argument.name));
        words.push(if argument.boolean {
            format!("u64::from({})", argument.name)
        } else {
            argument.name.clone()
        });
    }
    words.resize(6, String::from("0"));
    let answer = answer_type(row);
    let _ = writeln!(
        out,
        "{indent}pub fn {}({}) -> {}<{answer}> {{",
        function(row),
        parameters.join(", "),
        if row.kind == Kind::SystemCall {
            "Call"
        } else {
            "super::Call"
        },
    );
    let _ = writeln!(
        out,
        "{indent}    {}Call {{",
        if row.kind == Kind::SystemCall {
            ""
        } else {
            "super::"
        }
    );
    let _ = writeln!(out, "{indent}        number: {number},");
    let _ = writeln!(out, "{indent}        words: [{}],", words.join(", "));
    let _ = writeln!(out, "{indent}        answer: {},", answer_body(row));
    let _ = writeln!(out, "{indent}    }}");
    let _ = writeln!(out, "{indent}}}");
}

fn mask(out: &mut String, row: &Row) {
    let mut words: Vec<String> = row.arguments.iter().map(|a| bits(row, &a.bits)).collect();
    words.resize(4, String::from("0"));
    let _ = writeln!(
        out,
        "    /// The bits {} defines in argument words 2 to 5, word 2 first. A bit outside them has no\n    \
         /// meaning yet.",
        link(row)
    );
    let _ = writeln!(
        out,
        "    pub const {}_DEFINED_BITS: [u64; 4] = [{}];",
        row.name,
        words.join(", ")
    );
}

const HEAD: &str = r#"
/// One call, ready for the trap: the system call number, the six words the crate root's table
/// places in registers, and how to read the six words the kernel answers.
///
/// This crate makes no call itself. A program puts [`number`](Self::number) and
/// [`words`](Self::words) in the registers, traps, and passes the six words it gets back to
/// [`answer`](Self::answer).
#[derive(Debug, Clone, Copy)]
pub struct Call<A> {
    /// The system call number, one of the five at the crate root.
    pub number: u64,
    /// Words 0 to 5, in the registers the crate root's table names.
    pub words: [u64; 6],
    answer: fn([u64; 6]) -> A,
}

impl<A> Call<A> {
    /// Read the six words the kernel answered this call with.
    pub fn answer(&self, words: [u64; 6]) -> A {
        (self.answer)(words)
    }
}

/// Words 0 to 5 of an answer when word 0 is not negative; otherwise word 0 as the negative
/// [`Error`](crate::Error) code it is, which `Error::try_from` decodes.
fn refusal(words: [u64; 6]) -> Result<[u64; 6], i64> {
    let first = words[0] as i64;
    if first < 0 { Err(first) } else { Ok(words) }
}
"#;

/// The whole of `src/system_call.rs`.
pub fn system_call(rows: &[Row]) -> String {
    let mut out = String::from(BEGIN);
    out.push_str(HEAD);
    for row in rows
        .iter()
        .filter(|r| r.kind == Kind::SystemCall && r.has_wrapper())
    {
        out.push('\n');
        call(&mut out, "", row);
    }
    let mut objects: Vec<&str> = Vec::new();
    for row in rows
        .iter()
        .filter(|r| r.kind == Kind::Method && r.has_wrapper())
    {
        if !objects.contains(&row.object.as_str()) {
            objects.push(&row.object);
        }
    }
    for object in objects {
        let _ = writeln!(
            out,
            "\n/// The typed calls and defined bits of the methods [`{object}`](crate::{object}) numbers."
        );
        let _ = writeln!(out, "pub mod {object} {{");
        let mut first = true;
        for row in rows
            .iter()
            .filter(|r| r.object == object && r.has_wrapper())
        {
            if !first {
                out.push('\n');
            }
            first = false;
            mask(&mut out, row);
            out.push('\n');
            call(&mut out, "    ", row);
        }
        out.push_str("}\n");
    }
    out.push_str(END);
    out
}

fn expected_answer(row: &Row) -> String {
    match (row.returns, row.answers.len()) {
        (Returns::Never, _) => String::from("()"),
        (Returns::Refusal, _) => String::from("201"),
        (Returns::Answer, 0) => String::from("Ok(())"),
        (Returns::Answer, 1) => String::from("Ok(201)"),
        (Returns::Answer, n) => {
            let words: Vec<String> = (0..n).map(|i| (201 + i).to_string()).collect();
            format!("Ok([{}])", words.join(", "))
        }
    }
}

fn call_test(out: &mut String, row: &Row) {
    let (path, test, first) = if row.kind == Kind::SystemCall {
        (
            format!("system_call::{}", function(row)),
            function(row),
            100,
        )
    } else {
        let name = format!("{}_{}", row.object, function(row));
        (
            format!("system_call::{}::{}", row.object, function(row)),
            name,
            101,
        )
    };
    let mut arguments: Vec<String> = Vec::new();
    let mut words: Vec<String> = Vec::new();
    if row.kind == Kind::Method {
        arguments.push(String::from("100"));
        words.push(String::from("100"));
        words.push(format!("moho_abi::{}", row.path()));
    }
    for (i, argument) in row.arguments.iter().enumerate() {
        let n = first + i;
        arguments.push(if argument.boolean {
            String::from("true")
        } else {
            n.to_string()
        });
        words.push(if argument.boolean {
            String::from("1")
        } else {
            n.to_string()
        });
    }
    words.resize(6, String::from("0"));
    let number = if row.kind == Kind::SystemCall {
        row.constant()
    } else {
        String::from("SYSTEM_CALL_INVOKE")
    };
    let _ = writeln!(out, "\n#[test]\nfn {test}() {{");
    let _ = writeln!(out, "    let call = {path}({});", arguments.join(", "));
    let _ = writeln!(out, "    assert_eq!(call.number, moho_abi::{number});");
    let _ = writeln!(out, "    assert_eq!(call.words, [{}]);", words.join(", "));
    let _ = writeln!(
        out,
        "    assert_eq!(call.answer(ANSWER), {});",
        expected_answer(row)
    );
    let refused = match row.returns {
        Returns::Never => "()",
        Returns::Refusal => "-4",
        Returns::Answer => "Err(-4)",
    };
    let _ = writeln!(out, "    assert_eq!(call.answer([REFUSED; 6]), {refused});");
    out.push_str("}\n");
}

/// The whole of `tests/system_call.rs`: one test per typed call, each checking the words it puts
/// in registers and how it reads an answer and a refusal.
pub fn call_tests(rows: &[Row]) -> String {
    let mut out = String::from(BEGIN);
    out.push_str(
        "#![cfg_attr(rustfmt, rustfmt::skip)]\n\n\
         //! Each typed call places its arguments in the words the table gives them and reads its\n\
         //! answer back.\n\n\
         use moho_abi::system_call;\n\n\
         /// An answer whose words are told apart by value.\n\
         const ANSWER: [u64; 6] = [201, 202, 203, 204, 205, 206];\n\
         /// `BadPointer`, as the kernel leaves it in a register.\n\
         const REFUSED: u64 = -4_i64 as u64;\n\n\
         #[test]\n\
         fn a_word_0_of_zero_is_an_answer() {\n    \
             assert_eq!(system_call::yield_now().answer([0; 6]), Ok(()));\n\
         }\n",
    );
    for row in rows.iter().filter(|r| r.has_wrapper()) {
        call_test(&mut out, row);
    }
    out.push_str(END);
    out
}
