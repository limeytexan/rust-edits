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

use edit_matrix::*;

fn main() {
    println!("{:?}", create_edit_matrix("hey", "bee"));
}
