//! Terminal prompt utilities for interactive input collection.

use std::io::{self, BufRead, Write};

/// Prompts for a single line of input from a reader and returns it trimmed.
pub fn prompt_from_reader<R: BufRead>(reader: &mut R, label: &str) -> String {
    print!("{label}: ");
    io::stdout().flush().ok();

    let mut input = String::new();
    reader.read_line(&mut input).expect("failed to read input");
    input.trim().to_string()
}

/// Prompts for a single line of input from stdin and returns it trimmed.
#[allow(dead_code)]
pub fn prompt(label: &str) -> String {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    prompt_from_reader(&mut handle, label)
}

/// Like `prompt`, but treats empty input as "the user skipped this field".
pub fn prompt_optional_from_reader<R: BufRead>(reader: &mut R, label: &str) -> Option<String> {
    let value = prompt_from_reader(reader, &format!("{label} (opt)"));
    if value.is_empty() { None } else { Some(value) }
}

/// Like `prompt`, but treats empty input as "the user skipped this field".
#[allow(dead_code)]
pub fn prompt_optional(label: &str) -> Option<String> {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    prompt_optional_from_reader(&mut handle, label)
}

/// Repeatedly prompts for a value until the user enters nothing.
pub fn prompt_list_from_reader<R: BufRead>(reader: &mut R, label: &str) -> Vec<String> {
    let mut items = Vec::new();
    loop {
        let value = prompt_from_reader(reader, &format!("{label} (blank to stop)"));
        if value.is_empty() {
            break;
        }
        items.push(value);
    }
    items
}

/// Repeatedly prompts for a value until the user enters nothing.
#[allow(dead_code)]
pub fn prompt_list(label: &str) -> Vec<String> {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    prompt_list_from_reader(&mut handle, label)
}

/// Simple y/n prompt, defaulting to "no" on anything but an explicit y/yes.
pub fn confirm_from_reader<R: BufRead>(reader: &mut R, label: &str) -> bool {
    let value = prompt_from_reader(reader, &format!("{label} (y/N)"));
    value.eq_ignore_ascii_case("y") || value.eq_ignore_ascii_case("yes")
}

/// Simple y/n prompt, defaulting to "no" on anything but an explicit y/yes.
#[allow(dead_code)]
pub fn confirm(label: &str) -> bool {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    confirm_from_reader(&mut handle, label)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_prompt_from_reader() {
        let mut input = Cursor::new("hello world\n");
        let result = prompt_from_reader(&mut input, "Test");
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_prompt_optional_from_reader() {
        let mut input_some = Cursor::new("data\n");
        assert_eq!(
            prompt_optional_from_reader(&mut input_some, "Test"),
            Some("data".to_string())
        );

        let mut input_none = Cursor::new("\n");
        assert_eq!(prompt_optional_from_reader(&mut input_none, "Test"), None);
    }

    #[test]
    fn test_prompt_list_from_reader() {
        let mut input = Cursor::new("item1\nitem2\n\n");
        let items = prompt_list_from_reader(&mut input, "Items");
        assert_eq!(items, vec!["item1".to_string(), "item2".to_string()]);
    }

    #[test]
    fn test_confirm_from_reader() {
        let mut input_yes = Cursor::new("yes\n");
        assert!(confirm_from_reader(&mut input_yes, "Proceed"));

        let mut input_y = Cursor::new("y\n");
        assert!(confirm_from_reader(&mut input_y, "Proceed"));

        let mut input_no = Cursor::new("no\n");
        assert!(!confirm_from_reader(&mut input_no, "Proceed"));

        let mut input_other = Cursor::new("\n");
        assert!(!confirm_from_reader(&mut input_other, "Proceed"));
    }
}
