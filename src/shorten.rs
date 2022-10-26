// Size used to decide if a piece of text needs to be shortened
use crate::token::*;
use Token::*;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ShortenOptions {
    size: u8,
    text: String,
}

// Cut the shorten size in 2
fn half(s: &ShortenOptions) -> ShortenOptions {
    let mut result = s.clone();
    result.size = s.size / 2;
    return result;
}

// Shorten a piece of text that has already been tokenized
fn shorten_tokens(so: ShortenOptions, start: Token, end: Token, tokens: Vec<Token>) -> Vec<Token> {
    let mut to_shorten = vec![Start];
    to_shorten.extend(tokens);
    to_shorten.push(End);
    let start_clone = start.clone();
    let end_clone = end.clone();
    let delimited = split_on_delimiters(start, end, to_shorten);
    let mut result: Vec<Token> = vec![];
    for ts in delimited.iter() {
        match (ts.first(), ts.last()) {
          (Some(Start), _) => result.extend(shorten_left(so.clone(), ts.clone())),
          (_, Some(End)) => result.extend(shorten_right(so.clone(), ts.clone())),
          (Some(s), Some(e)) if *s == start_clone && *e == end_clone => result.extend(ts.clone()),
          (_, _) => result.extend(shorten_center(so.clone(), ts.clone())),
        }
    }
    return result;
}

// Split a list of tokens into several lists when a delimiter is found
// abcd[efgh]ijkl[mnop]qrst -> [abcd, [efgh], ijkl, [mnop], qrst]
fn split_on_delimiters(start: Token, end: Token, tokens: Vec<Token>) -> Vec<Vec<Token>> {
    let mut result: Vec<Vec<Token>> = vec![];
    for t in tokens.iter() {
        if *t == start {
            result.push(vec![start.clone()]);
        } else {
            if *t == end {
                update_last(&mut result, end.clone());
            } else {
                match result.last_mut() {
                    Some(ts) => match ts.last() {
                        Some(l) => {
                            if *l == end {
                                result.push(vec![t.clone()]);
                            } else {
                                update_last(&mut result, t.clone());
                            }
                        }
                        None => update_last(&mut result, t.clone()),
                    },
                    None => result.extend(vec![vec![t.clone()]]),
                }
            }
        }
    }
    return result;
}

fn update_last<T: Clone>(result: &mut Vec<Vec<T>>, t: T) -> () {
    match result.last_mut() {
        Some(l) => l.push(t.clone()),
        None => (),
    };
}

// Shorten some token on the left: ...tokens
fn shorten_left(so: ShortenOptions, original: Vec<Token>) -> Vec<Token> {
    if token_size(&original) > so.size.into() {
        let mut shortened = vec![Kept(so.text)];
        shortened.extend(
            original
                .iter()
                .skip(original.len() - usize::from(so.size))
                .cloned(),
        );
        return shortened;
    } else {
        return original.to_vec();
    }
}

// Shorten some token on the right: tokens...
fn shorten_right(so: ShortenOptions, original: Vec<Token>) -> Vec<Token> {
    if token_size(&original) > so.size.into() {
        let mut shortened = vec![];
        shortened.extend(original.iter().take(so.size.into()).cloned());
        shortened.push(Kept(so.text));
        return shortened;
    } else {
        return original.to_vec();
    }
}

// Shorten some token in the center: ...tokens...
fn shorten_center(so: ShortenOptions, original: Vec<Token>) -> Vec<Token> {
    if token_size(&original) > so.size.into() {
        let half_size = half(&so).size.into();
        let mut shortened = vec![];
        shortened.extend(original.iter().take(half_size).cloned());
        shortened.push(Kept(so.text));
        shortened.extend(original.iter().skip(original.len() - half_size).cloned());
        return shortened;
    } else {
        return original.to_vec();
    }
}

// Return the size of a list of tokens by only considering
// the strings we want to kept
fn token_size(ts: &Vec<Token>) -> usize {
    return ts
        .iter()
        .map(|t| match t {
            Kept(s) => s.len(),
            _ => 0,
        })
        .sum();
}

// TESTS

#[cfg(test)]
mod tests {
    use super::*;

    fn shorten_options() -> ShortenOptions {
        return ShortenOptions {
            size: 3,
            text: "...".to_string(),
        };
    }

    fn kept(s: &str) -> Token {
        return Kept(s.to_string());
    }
    fn delimiter(s: &str) -> Token {
        return Delimiter(s.to_string());
    }

    #[test]
    fn test_shorten_left() {
        assert_eq!(shorten_string_left(shorten_options(), "abcde"), "...cde");
    }
    #[test]
    fn test_shorten_right() {
        assert_eq!(shorten_string_right(shorten_options(), "abcde"), "abc...");
    }
    #[test]
    fn test_shorten_center() {
        assert_eq!(shorten_string_center(shorten_options(), "abcde"), "a...e");
    }
    #[test]
    fn test_split_on_delimiters() {
        let start = delimiter("[");
        let end = delimiter("]");
        // ab[cd]ef[g]h -> [ab, [cd], ef, [g], h]
        let delimited = vec![
            kept("a"),
            kept("b"),
            start.clone(),
            kept("c"),
            kept("d"),
            end.clone(),
            kept("e"),
            kept("f"),
            start.clone(),
            kept("g"),
            end.clone(),
            kept("h"),
        ];

        let expected = vec![
            vec![kept("a"), kept("b")],
            vec![start.clone(), kept("c"), kept("d"), end.clone()],
            vec![kept("e"), kept("f")],
            vec![start.clone(), kept("g"), end.clone()],
            vec![kept("h")],
        ];
        assert_eq!(
            split_on_delimiters(start.clone(), end.clone(), delimited),
            expected
        );
    }
    #[test]
    fn test_shorten() {
        // assert_eq!(shorten("abcd"), "abcd");
        assert_eq!(shorten("abcdefghijkl[mn]opqr"), "...hijkl[mn]opqr");
        // assert_eq!(shorten("abcdefghijkl[mn]".to_string()), "...hijkl[mn]");
        // assert_eq!(shorten("[mn]abcdefghijkl".to_string()), "[mn]abcde...");
        // assert_eq!(
        //     shorten("abcdefghijkl[mn]opqrstuv".to_string()),
        //     "...hijkl[mn]opqrs..."
        // );
        // assert_eq!(
        //     shorten("hijkl[zz]abcdefghijklmno[xx]abcde".to_string()),
        //     "hijkl[zz]ab...no[xx]abcde"
        // );
        // assert_eq!(
        //     shorten("hijkl[]xxabcdefghijklmno[]xxabcde".to_string()),
        //     "hijkl[]xx...no[]xxabc..."
        // );
        // assert_eq!(shorten("abcdef[]ghijkl".to_string()), "...bcdef[]ghijk...");
        // assert_eq!(
        //     shorten("abcdefg[zz]abcdefghijklmno[xx]abcdefg".to_string()),
        //     "...cdefg[zz]ab...no[xx]abcde..."
        // );
    }

    fn shorten(s: &str) -> String {
        let start = delimiter("[");
        let end = delimiter("]");
        let so = ShortenOptions {
            size: 5,
            text: "...".to_string(),
        };
        let tokens = to_tokens(s.to_string());
        return show_tokens(shorten_tokens(so, start, end, tokens));
    }
    fn shorten_string_right(so: ShortenOptions, s: &str) -> String {
        let tokens = to_tokens(s.to_string());
        return show_tokens(shorten_right(so, tokens));
    }
    fn shorten_string_left(so: ShortenOptions, s: &str) -> String {
        let tokens = to_tokens(s.to_string());
        return show_tokens(shorten_left(so, tokens));
    }
    fn shorten_string_center(so: ShortenOptions, s: &str) -> String {
        let tokens = to_tokens(s.to_string());
        return show_tokens(shorten_center(so, tokens));
    }
    fn to_tokens(s: String) -> Vec<Token> {
        let start = delimiter("[");
        let end = delimiter("]");
        let mut tokens: Vec<Token> = vec![];
        for c in s.chars() {
            if c == '[' {
                tokens.push(start.clone());
            } else if c == ']' {
                tokens.push(end.clone());
            } else {
                tokens.push(kept(c.to_string().as_str()));
            };
        }
        return tokens;
    }
}
