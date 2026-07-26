//! Terminal prompt utilities for interactive input collection.

use std::io::{self, Write};

/// Prompts for a single line of input and returns it trimmed. This is the
/// CLI's own concern — it's what stands in for a form field until the GUI
/// exists, so it stays out of tgk-core.
pub fn prompt(label: &str) -> String {
    print!("{label}: ");
    io::stdout().flush().ok();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
    input.trim().to_string()
}

/// Like `prompt`, but treats empty input as "the user skipped this field".
pub fn prompt_optional(label: &str) -> Option<String> {
    let value = prompt(&format!("{label} (opt)"));
    if value.is_empty() { None } else { Some(value) }
}

/// Repeatedly prompts for a value (e.g. one email at a time) until the user
/// enters nothing, then returns everything collected. Used for fields where
/// someone might have more than one (emails, phone numbers, relatives...).
pub fn prompt_list(label: &str) -> Vec<String> {
    let mut items = Vec::new();
    loop {
        let value = prompt(&format!("{label} (blank to stop)"));
        if value.is_empty() {
            break;
        }
        items.push(value);
    }
    items
}

/// Simple y/n prompt, defaulting to "no" on anything but an explicit y/yes.
pub fn confirm(label: &str) -> bool {
    let value = prompt(&format!("{label} (y/N)"));
    value.eq_ignore_ascii_case("y") || value.eq_ignore_ascii_case("yes")
}
