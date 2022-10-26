//  This module provides a 'show_distance' function showing the differences between 2 pieces of text
//   using the Levenshtein distance. That distance is defined as the minimum number of edits: insertions, deletions, substitutions
//   to go from one text to another.
//
//   Several options are available to customize this processing:
//
//     - split size: texts are broken into new lines first. Then if the texts are too large there are split into smaller pieces
//       in order to compute their difference. This is done in order to reduce the size of the edit matrix which is used to compute all the edit costs
//       the default is 200
//
//     - separators: opening and closing pieces of text (brackets by default) used to highlight a difference
//
//     - shorten size: there is the possibly to display mostly the differences with a bit of context around if the input text is too large.
//       The text gets elided around separators if it gets greater than the shorten size (the default is 20)
//
//     - shorten text: the text to use when eliding characters in the original text (the default is "...")
//
//     - display edit operations: edit operations, insert/delete/substitute/keep can be annotated if necessary
//
// Here are some examples:
//
// @
// import Data.Text.Edits
//
// -- "between the e and the n the letter i was added"
// showDistance "kitten" "kittein" === "kitte[+i]n"
//
// -- "at the end of the text 3 letters have been deleted"
// showDistance "kitten" "kit" === "kit[-t-e-n]"
//
// -- "between the t and the n 2 letters have been modified"
// showDistance "kitten" "kitsin" === "kit[~t/s~e/i]"
// @
use crate::costs::*;
use crate::difference::*;
use crate::edit_matrix::*;
use crate::edit_operation::*;

// Size to use when splitting a large piece of text
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SplitSize {
    pub split_size: usize,
}

// Default split size
fn default_split_size() -> SplitSize {
    SplitSize { split_size: 200 }
}

// Show the distance between 2 pieces of text
pub fn show_distance(s1: String, s2: String) -> String {
    show_distance_with(default_split_size(), default_display_options(), s1, s2)
}

// Show the distance between 2 pieces of text with colors instead of symbols
pub fn show_distance_colored(s1: String, s2: String) -> String {
    let mut options = default_display_options();
    options.display_edit_operation = colored_display_edit_operation;
    show_distance_with(default_split_size(), options, s1, s2)
}

// Show the distance between 2 pieces of text and specify splitting / display options
pub fn show_distance_with(
    split_size: SplitSize,
    display_options: DisplayOptions,
    s1: String,
    s2: String,
) -> String {
    let mut result: Vec<String> = vec![];
    let splitted1 = split(split_size.clone(), s1);
    let splitted2 = split(split_size, s2);
    let zipped = splitted1.iter().zip(splitted2.iter());
    for (line1, line2) in zipped {
        let operations = levenshtein_operations(line1.clone(), line2.clone());
        result.push(display_diffs(display_options.clone(), operations));
    }
    result.concat()
}

// Return the list of operations necessary to go from one piece of text to another
// using the Levenshtein distance
pub fn levenshtein_operations(s1: String, s2: String) -> Vec<EditOperation<char>> {
    let matrix = create_edit_matrix(&levenshtein_costs(), s1.to_string(), s2.to_string());
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
        assert_eq!(show_distance("k".to_string(), "l".to_string()), "[~k/l]");
        assert_eq!(show_distance("ki".to_string(), "ka".to_string()), "k[~i/a]");
        assert_eq!(
            show_distance("kitte".to_string(), "kittei".to_string()),
            "kitte[+i]"
        );
        assert_eq!(
            show_distance("kitten".to_string(), "kittein".to_string()),
            "kitte[+i]n"
        );
        assert_eq!(
            show_distance("kitten".to_string(), "kit".to_string()),
            "kit[-t-e-n]"
        );
        assert_eq!(
            show_distance("kit".to_string(), "kitten".to_string()),
            "kit[+t+e+n]"
        );
        assert_eq!(
            show_distance("kitten".to_string(), "kitsin".to_string()),
            "kit[~t/s~e/i]n"
        );
        assert_eq!(
            show_distance("kitte".to_string(), "kitte".to_string()),
            "kitte"
        );
    }
}
