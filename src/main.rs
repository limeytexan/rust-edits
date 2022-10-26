#![allow(dead_code)]
#![allow(unused_variables)]
mod color;
mod costs;
mod difference;
mod edit_matrix;
mod edit_operation;
mod matrix;
mod shorten;
mod token;
mod edits;

use edit_matrix::*;
use costs::*;

fn main() {
    println!("{:?}", create_edit_matrix(&levenshtein_costs(), "hey".to_string(), "bee".to_string()));
}
