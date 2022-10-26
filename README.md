[![Workflow Status](https://github.com/etorreborre/rust-edits/workflows/CI/badge.svg)](https://github.com/etorreborre/rust-edits/actions?query=workflow%3A%22CI%22)

This library provides functions to show the differences between two strings.
It uses the [Levenshtein distance](https://en.wikipedia.org/wiki/Levenshtein_distance) to compute
the minimum number of edit operations: insertions, deletions, substitutions needed to go from one string to the other.

Several options are available to customize this processing:

  - `split_size` strings are broken into a list of lines if they contain any new lines first.
     Then if any line is too large, according to the `split_size` it is broken into smaller pieces for comparison
     This is done in order to reduce the size of the edit matrix which is used to compute all the edit costs
     The default is 200

  - `separators` opening and closing pieces of text (brackets by default) used to highlight a difference

  - `shorten size` there is the possibly to display mostly the differences with a bit of context around if the input text is too large.
      The string get elided around separators if it gets greater than the `shorten_size` (the default is 20)

  - `shorten_text` the string to use when eliding characters in the original string (the default is `"..."`)

  - `display_edit_operations` a function to specify how edit operations, insert/delete/substitute/keep are represented

 Here are some examples:
```rust
use edits::edits::*;

// "between the e and the n the letter i was added"
assert_eq!(show_distance("kitten", "kittein"), "kitte[+i]n");

// "at the end of the text 3 letters have been deleted"
assert_eq!(show_distance("kitten", "kit"), "kit[-t-e-n]");

// "between the t and the n 2 letters have been modified"
assert_eq!(show_distance("kitten", "kitsin"), "kit[~t/s~e/i]n");

// "between the t and the n 2 letters have been modified"
let my_split_size = SplitSize { split_size: 300 };
assert_eq!(show_distance_with(my_split_size, default_display_options(), "kitten", "kitsin"), "kit[~t/s~e/i]n");
```

The output can also be coloured. For example:
<img src="doc/images/example.jpg" border="0"/>
