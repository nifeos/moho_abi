//! The method table is the source of the system call surface: this test writes the constants of
//! every object module, `src/system_call.rs` and `tests/system_call.rs` from `method-table.toml`,
//! and fails when the committed files differ, so a hand edit to a generated part fails here.
//! `MOHO_ABI_REGENERATE=1 cargo test --test method_table` writes the files instead.
//!
//! The files are committed rather than written by a build script because a consumer may copy this
//! crate's source file by file, with no build step to run one.

mod model;
mod render;
mod table;

use model::{Kind, Row};
use std::path::PathBuf;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read(path: &str) -> String {
    std::fs::read_to_string(root().join(path)).unwrap_or_else(|e| panic!("cannot read {path}: {e}"))
}

fn rows() -> Vec<Row> {
    let entries = table::parse(&read("method-table.toml")).unwrap_or_else(|e| panic!("{e}"));
    entries
        .iter()
        .map(|entry| model::row(entry).unwrap_or_else(|e| panic!("{e}")))
        .collect()
}

/// Every generated file, as `(path, the text the table writes)`.
fn expected(rows: &[Row]) -> Vec<(String, String)> {
    let mut objects: Vec<&str> = Vec::new();
    for row in rows {
        if !objects.contains(&row.object.as_str()) {
            objects.push(&row.object);
        }
    }
    let mut files = Vec::new();
    for object in objects {
        let path = if object.is_empty() {
            String::from("src/lib.rs")
        } else {
            format!("src/{object}.rs")
        };
        let part = render::constants(rows, object);
        let text = render::splice(&path, &read(&path), &part).unwrap_or_else(|e| panic!("{e}"));
        files.push((path, text));
    }
    files.push((
        String::from("src/system_call.rs"),
        render::system_call(rows),
    ));
    files.push((
        String::from("tests/system_call.rs"),
        render::call_tests(rows),
    ));
    files
}

#[test]
fn the_generated_source_matches_the_table() {
    let regenerate = std::env::var_os("MOHO_ABI_REGENERATE").is_some();
    let mut stale = Vec::new();
    for (path, text) in expected(&rows()) {
        let now = std::fs::read_to_string(root().join(&path)).unwrap_or_default();
        if now == text {
            continue;
        }
        if regenerate {
            std::fs::write(root().join(&path), text).unwrap_or_else(|e| panic!("{path}: {e}"));
        } else {
            stale.push(path);
        }
    }
    assert!(
        stale.is_empty(),
        "these files differ from what method-table.toml writes: {stale:?}. Edit the table, not \
         the file, and run `MOHO_ABI_REGENERATE=1 cargo test --test method_table`"
    );
}

/// The table's numbers are the pinned ones: each row's line is in the hand-kept pinned lists, so
/// a renumber in the table fails there as a hand renumber would.
#[test]
fn every_number_in_the_table_is_pinned() {
    let tests = read("src/tests.rs");
    let record = read("pinned-at-version-1.txt");
    for row in rows() {
        let line = render::pinned_line(&row);
        assert!(
            tests.lines().any(|l| l.trim() == line),
            "src/tests.rs lacks `{line}`"
        );
        assert!(
            record.lines().any(|l| l.trim() == line),
            "pinned-at-version-1.txt lacks `{line}`"
        );
    }
}

/// Two methods of one object never share a number; a named value is not a method and may.
#[test]
fn no_two_methods_of_an_object_share_a_number() {
    let rows = rows();
    for (i, a) in rows.iter().enumerate() {
        for b in &rows[i + 1..] {
            let methods = a.kind != Kind::Value && b.kind != Kind::Value;
            assert!(
                !(methods && a.object == b.object && a.number == b.number),
                "{} and {} share {}",
                a.path(),
                b.path(),
                a.number
            );
        }
    }
}

/// The reader refuses what is outside its subset, naming the line, rather than reading it some
/// other way than a TOML reader would.
#[test]
fn the_table_reader_refuses_what_it_does_not_read() {
    for (text, refusal) in [
        ("x = 1\n", "a key before the first"),
        ("[[method]]\nx = 1\nx = 2\n", "a key given twice"),
        ("[[method]]\nx = 'single'\n", "expected a value"),
        ("[[method]]\nx = \"a\\tb\"\n", "an escape other than"),
        ("[[method]]\nx = 1 2\n", "text after the value"),
        (
            "[[method]]\n\nx = '''\nopen\n",
            "unterminated multi-line string",
        ),
    ] {
        let Err(e) = table::parse(text) else {
            panic!("{text:?} was read");
        };
        assert!(e.contains(refusal), "{text:?}: {e}");
    }
    let entries =
        table::parse("[[value]]\nx = 0xFF_FF # a comment\ny = [{ a = true }, \"b\",]\n").unwrap();
    assert_eq!(entries[0].line, 1);
    assert_eq!(
        entries[0].keys,
        [
            (String::from("x"), table::Value::Number(0xFFFF)),
            (
                String::from("y"),
                table::Value::List(vec![
                    table::Value::Table(vec![(String::from("a"), table::Value::Flag(true))]),
                    table::Value::Text(String::from("b")),
                ])
            ),
        ]
    );
}
