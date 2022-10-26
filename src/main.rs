#![allow(dead_code)]
#![allow(unused_variables)]
mod costs;
mod color;
use crate::costs::*;
use crate::costs::Cost::*;

fn main() {
    println!("{:?}", create_edit_matrix("hey", "bee"));
}

#[derive(Debug, Clone, PartialEq)]
struct Matrix<T: Sized> {
    rows: Vec<Vec<T>>,
}

impl<T> Matrix<T> {
    pub fn get_value(&self, i: usize, j: usize) -> Option<&T> {
        return self.rows.get(i)?.get(j);
    }
    pub fn set_value(&mut self, i: usize, j: usize, t: T) -> () {
        match self.rows.get(i) {
            None => (),
            Some(v) => match v.get(j) {
                None => (),
                Some(_) => self.rows[i][j] = t,
            },
        }
    }
}

fn init_matrix<T: Clone>(rows_nb: usize, cols_nb: usize, t: T) -> Matrix<T> {
    let row: Vec<T> = vec![t; cols_nb];
    let matrix: Matrix<T> = Matrix {
        rows: vec![row; rows_nb],
    };

    return matrix;
}

fn test1() -> () {
    let mut matrix = init_matrix(1, 1, NoAction(0));
    for i in 0..10 {
        let cost = use_matrix(i, &matrix);
        matrix.set_value(0, 0, cost)
    }
}

fn use_matrix(i: usize, matrix: &Matrix<Cost>) -> Cost {
    return match matrix.get_value(0, 1) {
        Some(v) => v.clone(),
        None => NoAction(0),
    };
}

fn create_edit_matrix(str1: &str, str2: &str) -> Matrix<Cost> {
    let mut matrix = init_matrix(str1.len(), str2.len(), NoAction(0));
    let coordinates = cartesian(str1.len(), str2.len());

    for ij in coordinates {
        let (i, j) = ij;
        let new_cost: Cost = if i == 0 {
            Insertion(j)
        } else if j == 0 {
            Deletion(i)
        } else {
            match cost_of(str1, str2, i, j, &matrix).clone() {
                Some(c) => c,
                _ => NoAction(0),
            }
        };
        matrix.set_value(i, j, new_cost);
    }

    return matrix;
}

// Return the cartesian product of n x m elements, 0-indexed
fn cartesian(n: usize, m: usize) -> Vec<(usize, usize)> {
    let mut result = vec![];
    for i in 0..n {
        for j in 0..m {
            result.push((i, j));
        }
    }
    return result;
}

// compute the cost of going from as1[i] to as2[j], knowing the existing costs
//  (i-1, j-1) (i-1, j)
//  (i, j-1)   (i, j)
//
// going from (i-1, j) to (i, j) means that we delete as1[i]
// going from (i-1, j-1) to (i, j) means that we substitute as1[i] with as2[j]
// going from (i, j-1) to (i, j) means that we insert as2[j]
fn cost_of(str1: &str, str2: &str, i: usize, j: usize, matrix: &Matrix<Cost>) -> Option<Cost> {
    let i1 = i - 1;
    let j1 = j - 1;
    let i1j = matrix.get_value(i1, j)?;
    let i1j1 = matrix.get_value(i1, j1)?;
    let ij1 = matrix.get_value(i, j1)?;
    let v1 = str1.chars().nth(i1)?;
    let v2 = str2.chars().nth(j1)?;

    let result = LevenshteinCosts::lower_cost(
        &v1,
        &v2,
        i1j.cost() + 1,                             // suppression
        i1j1.cost() + if v1 == v2 { 0 } else { 1 }, // substitution
        ij1.cost() + 1,
    ); // insertion
       // in case of a substitution if the resulting cost of (i, j) is the same as (i-1, j-1)
       // this means that we have substituted the same letter and it is the same as doing no action
    match result {
        Substitution(_) => {
            if i1j1.cost() == result.cost() {
                Some(NoAction(result.cost()))
            } else {
                Some(result)
            }
        }
        _ => Some(result),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cartesian() {
        assert_eq!(
            cartesian(2, 3),
            vec![(0, 0), (0, 1), (0, 2), (1, 0), (1, 1), (1, 2)]
        );
    }
    #[test]
    fn test_create_edit_matrix() {
        assert_eq!(
            create_edit_matrix("hello", "hey"),
            Matrix {
                rows: vec![
                    vec![Insertion(0), Insertion(1), Insertion(2)],
                    vec![Deletion(1), NoAction(0), Insertion(1)],
                    vec![Deletion(2), Deletion(1), NoAction(0)],
                    vec![Deletion(3), Deletion(2), Deletion(1)],
                    vec![Deletion(4), Deletion(3), Deletion(2)]
                ]
            }
        );
    }
}

// createEditMatrix :: Costs a -> [a] -> [a] -> Matrix Cost
// createEditMatrix costs as1 as2 = do
//   let initialMatrix = M.matrix (length as1 + 1) (length as2 + 1) (const $ NoAction 0)
//   let coordinates = cartesian (length as1) (length as2)
//   foldl'
//       ( \ma (i, j) ->
//           let newCost
//                 | i == 0 = Insertion j -- no more letters for as1, we do j insertions
//                 | j == 0 = Deletion i -- no more letters for as2, we do i suppressions
//                 | otherwise = costOf i j ma -- otherwise we compute the cost and operation to go from as1[i] to as2[j]
//            in setElement newCost i j ma
//       )
//       initialMatrix
//       coordinates
//   where
//     (vs1, vs2) = (V.fromList as1, V.fromList as2)

//     -- compute the cartesian product of the [0..i] x [0..j] lists
//     cartesian :: Int -> Int -> [(Int, Int)]
//     cartesian m n = [(i, j) | i <- [0 .. m], j <- [0 .. n]]

//     -- compute the cost of going from as1[i] to as2[j], knowing the existing costs
//     --  (i-1, j-1) (i-1, j)
//     --  (i, j-1)   (i, j)
//     --
//     -- going from (i-1, j) to (i, j) means that we delete as1[i]
//     -- going from (i-1, j-1) to (i, j) means that we substitute as1[i] with as2[j]
//     -- going from (i, j-1) to (i, j) means that we insert as2[j]
//     costOf :: Int -> Int -> Matrix Cost -> Cost
//     costOf i j matrix = do
//       let i1 = i - 1
//       let j1 = j - 1
//       let i1j =  getElement i1 j matrix
//       let i1j1 = getElement i1 j1 matrix
//       let ij1 =  getElement i j1 matrix
//       let result =
//             lowerCost
//               costs
//               (vs1 ! i1)
//               (vs2 ! j1)
//               (cost i1j + deletionCost costs (vs1 ! i1)) -- suppression
//               (cost i1j1 + substitutionCost costs (vs1 ! i1) (vs2 ! j1)) -- substitution
//               (cost ij1 + insertionCost costs (vs2 ! j1)) -- insertion
//       -- in case of a substitution if the resulting cost of (i, j) is the same as (i-1, j-1)
//       -- this means that we have substituted the same letter and it is the same as doing no action
//       case result of
//         Substitution {} | cost i1j1 == cost result -> NoAction (cost result)
//         _ -> result
