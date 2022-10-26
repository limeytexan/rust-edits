pub use crate::difference::*;
use crate::costs::*;
use crate::edit_matrix::*;
use crate::edit_operation::*;

// Show the distance between 2 pieces of text
pub fn show_distance<S1: Into<String>, S2: Into<String>>(s1: S1, s2: S2) -> String {
    show_distance_with(default_split_size(), default_display_options(), s1, s2)
}

// Show the distance between 2 pieces of text with colors instead of symbols
pub fn show_distance_colored<S1: Into<String>, S2: Into<String>>(s1: S1, s2: S2) -> String {
    let mut options = default_display_options();
    options.display_edit_operation = colored_display_edit_operation;
    show_distance_with(default_split_size(), options, s1, s2)
}

// Show the distance between 2 pieces of text and specify splitting / display options
pub fn show_distance_with<S1: Into<String>, S2: Into<String>>(
    split_size: SplitSize,
    display_options: DisplayOptions,
    s1: S1,
    s2: S2,
) -> String {
    let mut result: Vec<String> = vec![];
    let splitted1 = split(split_size.clone(), s1.into());
    let splitted2 = split(split_size, s2.into());
    let zipped = splitted1.iter().zip(splitted2.iter());
    for (line1, line2) in zipped {
        let operations = levenshtein_operations(line1.clone(), line2.clone());
        result.push(display_diffs(display_options.clone(), operations));
    }
    result.concat()
}

// Size to use when splitting a large piece of text
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SplitSize {
    pub split_size: usize,
}

// Default split size
fn default_split_size() -> SplitSize {
    SplitSize { split_size: 200 }
}

// Return the list of operations necessary to go from one piece of text to another
// using the Levenshtein distance
fn levenshtein_operations(s1: String, s2: String) -> Vec<EditOperation<char>> {
    let matrix = create_edit_matrix(&levenshtein_costs(), s1.clone(), s2.clone());
    make_edit_operations(s1.chars().collect(), s2.chars().collect(), matrix)
}

// Split texts and apply the difference on each part
// Split a text on newlines then split each line on a maximum split size
// We then perform the edit distance algorithm on smaller sizes of text in order to control memory and CPU
fn split(split_size: SplitSize, s: String) -> Vec<String> {
    s.split('\n')
        .map(|s2| split_to_size(split_size.clone(), s2.to_string()))
        .collect::<Vec<Vec<String>>>()
        .concat()
}

// Split a text on a maximum split size
fn split_to_size(split_size: SplitSize, s: String) -> Vec<String> {
    let n = split_size.split_size;
    if s.len() <= n {
        vec![s]
    } else {
        let mut result: Vec<String> = vec![];
        result.push(s.chars().take(n).collect());
        result.extend(split_to_size(split_size, s.chars().skip(n).collect()));
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use EditOperation::*;

    #[test]
    fn test_split_to_size() {
        let split_size = SplitSize { split_size: 5 };
        assert_eq!(
            split_to_size(split_size, "abcdefghij".to_string()),
            vec!["abcde".to_string(), "fghij".to_string()]
        );
    }
    #[test]
    fn test_levenshtein_operations() {
        assert_eq!(
            levenshtein_operations("kitte".to_string(), "kittei".to_string()),
            vec![
                Keep('k'),
                Keep('i'),
                Keep('t'),
                Keep('t'),
                Keep('e'),
                Insert('i')
            ]
        );
    }
    #[test]
    fn test_show_distance() {
        assert_eq!(show_distance("k", "l"), "[~k/l]");
        assert_eq!(show_distance("ki", "ka"), "k[~i/a]");
        assert_eq!(
            show_distance("kitte", "kittei"),
            "kitte[+i]"
        );
        assert_eq!(
            show_distance("kitten", "kittein"),
            "kitte[+i]n"
        );
        assert_eq!(
            show_distance("kitten", "kit"),
            "kit[-t-e-n]"
        );
        assert_eq!(
            show_distance("kit", "kitten"),
            "kit[+t+e+n]"
        );
        assert_eq!(
            show_distance("kitten", "kitsin"),
            "kit[~t/s~e/i]n"
        );
        assert_eq!(
            show_distance("kitte", "kitte"),
            "kitte"
        );
    }
}
