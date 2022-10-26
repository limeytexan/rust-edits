#![allow(dead_code)]
#![allow(unused_variables)]
mod color;
mod costs;
mod shorten;
mod token;
mod matrix;
mod edit_matrix;

use edit_matrix::*;

fn main() {
    println!("{:?}", create_edit_matrix("hey", "bee"));
}
