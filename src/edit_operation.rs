use crate::costs::*;
use crate::matrix::*;
use Cost::*;
use EditOperation::*;

// Atomic operation required to edit a piece of text
//   at a given position in the EditMatrix
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum EditOperation<T> {
    Insert(T),
    Delete(T),
    Substitute(T, T),
    Keep(T),
}

// Inverse of an edit operation. It is used
// to display not only how to go from text1 to text2 but also from text2 to text1
pub fn inverse<T>(e: EditOperation<T>) -> EditOperation<T> {
    match e {
        Insert(t) => Delete(t),
        Delete(t) => Insert(t),
        Substitute(t1, t2) => Substitute(t2, t1),
        Keep(t) => Keep(t),
    }
}

// From the original lists of characters, given the cost matrix
// return a list of edit operations allowing to edit one text and eventually get the second one
pub fn make_edit_operations<T: Clone>(
    ts1: Vec<T>,
    ts2: Vec<T>,
    matrix: Matrix<Cost>,
) -> Vec<EditOperation<T>> {
    let mut result: Vec<EditOperation<T>> = vec![];
    if ts1.is_empty() || ts2.is_empty() {
        return result;
    }

    for i in 0..(ts1.len()) {
        for j in 0..(ts2.len()) {
            if let Some(op) = matrix.get_value(i, j) {
                let dist = op.cost();
                if i == 0 && j == 0 {
                    if dist == 0 {
                        if let Some(t) = ts1.first() {
                            result.push(Keep((*t).clone()));
                        }
                    } else if let (Some(t1), Some(t2)) = (ts1.first(), ts2.first()) {
                        result.push(Substitute((*t1).clone(), (*t2).clone()));
                    }
                } else if j == ts2.len() - 1 {
                    result.extend(ts1.iter().take(i).map(|t| Delete((*t).clone())));
                } else if i == ts1.len() - 1 {
                    result.extend(ts2.iter().take(j).map(|t| Insert((*t).clone())));
                } else {
                    match op {
                        Insertion(a) => result.push(Insert(ts2[j - 1].clone())),
                        Deletion(a) => result.push(Delete(ts1[i - 1].clone())),
                        Substitution(a) => {
                            result.push(Substitute(ts1[i - 1].clone(), ts2[j - 1].clone()))
                        }
                        _ => result.push(Keep(ts1[i - 1].clone())),
                    }
                }
            }
        }
    }

    result
}
