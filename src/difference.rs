use crate::color::*;
use crate::edit_operation::*;
use crate::shorten::*;
use crate::token::*;
use Color::*;
use EditOperation::*;
use Token::*;

// Separators are used to highlight a difference between 2 pieces of text
// for example
#[derive(PartialEq, Eq, Debug)]
pub struct Separators {
    start_separator: String,
    end_separator: String,
}

// Make parens separators
pub fn parens_separators() -> Separators {
    make_char_separators('(', ')')
}

pub fn brackets_separators() -> Separators {
    make_char_separators('[', ']')
}

// Make separators with simple Chars
pub fn make_char_separators(c1: char, c2: char) -> Separators {
    Separators {
        start_separator: c1.to_string(),
        end_separator: c2.to_string(),
    }
}

// Options to use for displaying differences
pub struct DisplayOptions {
    separators: Separators,
    shorten_options: ShortenOptions,
    display_edit_operation: fn(EditOperation<char>) -> String,
}

// Default display options
pub fn default_display_options() -> DisplayOptions {
    DisplayOptions {
        separators: brackets_separators(),
        shorten_options: ShortenOptions {
            size: 20,
            text: "...".to_string(),
        },
        display_edit_operation: default_display_edit_operations,
    }
}

// Display an edit operation by prepending a symbol showing which operation is used
fn default_display_edit_operations(e: EditOperation<char>) -> String {
    match e {
        Insert(c) => format!("+{}", c.to_string()),
        Delete(c) => format!("-{}", c.to_string()),
        Substitute(c1, c2) => format!("~{}{}", c1.to_string(), c2.to_string()),
        Keep(c) => c.to_string(),
    }
}

// Display an edit operation using ascii colors: green = added, red = removed, blue = substituted
fn colored_display_edit_operation(e: EditOperation<char>) -> String {
    match e {
        Insert(c) => c.color_as(Green),
        Delete(c) => c.color_as(Red),
        Substitute(c, _) => c.color_as(Cyan),
        Keep(c) => c.to_string(),
    }
}
// Show the differences by enclosing them in separators
// Additionally shorten the text outside the separators if it is too long
fn display_diffs(options: DisplayOptions, operations: Vec<EditOperation<char>>) -> String {
    let start = options.separators.start_separator;
    let end = options.separators.end_separator;
    let mut result: Vec<Token> = vec![];
    let mut different = false;
    for operation in operations {
        match operation {
            Insert(_) | Delete(_) | Substitute(_, _) => {
                different = true;
                if !different {
                    result.push(Delimiter(start.clone()))
                };
                result.push(Kept((options.display_edit_operation)(operation)))
            }
            Keep(_) => {
                different = false;
                if different {
                    result.push(Delimiter(end.clone()))
                };
                result.push(Kept((options.display_edit_operation)(operation)))
            }
        }
    }
    if different {
        result.push(Delimiter(end.clone()));
    }

    let full_result: Vec<String> = shorten_tokens(
        options.shorten_options,
        Delimiter(start),
        Delimiter(end),
        result,
    )
    .iter()
    .map(|token| show_token(token))
    .collect();
    return full_result.join("");
}
