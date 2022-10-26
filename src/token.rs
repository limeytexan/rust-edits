// This module helps parsing some text with delimited differences

// A Token is used to enclose a piece of text to compare and delimiters showing where the text is different from another piece of text
//   Start / End are markers for the beginning and end of that text
#[derive(Clone)]
pub enum Token {
    Kept(String),
    Delimiter(String),
    Start,
    End,
}

use Token::*;

// Show a Token by skipping Start/End if present
pub fn show_token(t: &Token) -> String {
    match t {
        Kept(s) => s.clone(),
        Delimiter(s) => s.clone(),
        Start => String::new(),
        End => String::new(),
    }
}

// Show a list of tokens.
// Start/End are skipped
pub fn show_tokens<'a>(ts: Vec<&'a Token>) -> String {
    //T.concat . fmap showToken
    let mut s: Vec<String> = vec![];
    for t in ts.iter() {
        s.push(show_token(&t))
    }
    return s.join("");
}
