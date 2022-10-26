//! This crate provides a way to compare 2 strings and display their differences
/// ```
/// use edits::edits::*;
///
/// // "between the e and the n the letter i was added"
/// assert_eq!(show_distance("kitten", "kittein"), "kitte[+i]n");
///
/// // "at the end of the text 3 letters have been deleted"
/// assert_eq!(show_distance("kitten", "kit"), "kit[-t-e-n]");
///
/// // "between the t and the n 2 letters have been modified"
/// assert_eq!(show_distance("kitten", "kitsin"), "kit[~t/s~e/i]n");
///
/// // "between the t and the n 2 letters have been modified"
/// let my_split_size = SplitSize { split_size: 300 };
/// assert_eq!(show_distance_with(my_split_size, default_display_options(), "kitten", "kitsin"), "kit[~t/s~e/i]n");
/// ```
pub mod color;
pub mod costs;
pub mod difference;
pub mod edit_matrix;
pub mod edit_operation;
pub mod matrix;
pub mod shorten;
pub mod token;
pub mod edits;
