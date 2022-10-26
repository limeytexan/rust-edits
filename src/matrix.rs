use core::mem::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Matrix<T: Sized> {
    pub rows: Vec<Vec<T>>,
}

impl<T> Matrix<T> {
    // Return a value if it exists at coordinates i and j
    pub fn get_value(&self, i: usize, j: usize) -> Option<&T> {
        self.rows.get(i)?.get(j)
    }

    // Set a value at coordinates i and j.
    // Returns Some(())) if there is such a value and None otherwise
    pub fn set_value(&mut self, i: usize, j: usize, t: T) -> Option<T> {
        self.rows
            .get_mut(i)
            .and_then(|v| v.get_mut(j))
            .map(|val| replace(val, t))
    }
}

pub fn init_matrix<T: Clone>(rows_nb: usize, cols_nb: usize, t: T) -> Matrix<T> {
    Matrix {
        rows: vec![vec![t; cols_nb]; rows_nb],
    }
}
