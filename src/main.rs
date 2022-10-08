#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    println!("{:?}", create_edit_matrix("hey", "bee"));
}

#[derive(Debug)]
struct Costs {}

#[derive(Debug, Clone)]
enum Cost {
    Insertion(u32),
    Deletion(u32),
    Substitution(u32),
    NoAction,
}

// data Cost
//   = Insertion Int
//   | Deletion Int
//   | Substitution Int
//   | NoAction Int
//   deriving (Eq, Show)

#[derive(Debug, Clone)]
struct Matrix<T: Sized> {
    rows: Vec<Vec<T>>,
}

fn init_matrix<T : Clone>(rows_nb: usize, cols_nb: usize, t: T) -> Matrix<T> {
    let row: Vec<T> = vec![t; cols_nb];
    let matrix: Matrix<T> = Matrix {
        rows: vec![row; rows_nb]
    };

    return matrix;
}

fn create_edit_matrix(string1: &str, string2: &str) -> Matrix<Cost> {
    let initial_matrix = init_matrix(string1.len(), string2.len(), Cost::NoAction);

    return initial_matrix;
}
