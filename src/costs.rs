use Cost::*;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Cost {
    Insertion(usize),
    Deletion(usize),
    Substitution(usize),
    NoAction(usize),
}

// Don't prefix Cost variants

impl Cost {
    pub fn cost(self) -> usize {
        match self {
            Insertion(c) => c,
            Deletion(c) => c,
            Substitution(c) => c,
            NoAction(c) => c,
        }
    }
}

pub fn show_cost(c: &Cost) -> String {
    match c {
        Insertion(c) => format!("+{}", c),
        Deletion(c) => format!("-{}", c),
        Substitution(c) => format!("~{}", c),
        NoAction(c) => format!("o{}", c),
    }
}

// This component contains functions to evaluate the cost of
// substituting, inserting, deleting an element
pub trait Costs<T>: Copy {
    fn insertion_cost(self, t: &T) -> usize;
    fn deletion_cost(self, t: &T) -> usize;
    fn substitution_cost(self, t1: &T, t2: &T) -> usize;
    fn lower_cost(self, t1: &T, t2: &T, ins: usize, del: usize, sub: usize) -> Cost;
}

// Implementation of the Costs trait for the Levenshtein distance

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct LevenshteinCosts {}

impl Costs<char> for LevenshteinCosts {
    fn insertion_cost(self, _t: &char) -> usize {
        1
    }
    fn deletion_cost(self, _t: &char) -> usize {
        1
    }
    fn substitution_cost(self, t1: &char, t2: &char) -> usize {
        if t1 == t2 {
            0
        } else {
            1
        }
    }
    fn lower_cost(self, t1: &char, t2: &char, ins: usize, del: usize, sub: usize) -> Cost {
        let (op_ins, op_del, op_sub) = (Insertion(ins), Deletion(del), Substitution(sub));
        if ins < del {
            if (ins < sub) || (ins == sub && t1 == t2) {
                op_ins
            } else {
                op_sub
            }
        } else if (del < sub) || (del == sub && t1 == t2) {
            op_del
        } else {
            op_sub
        }
    }
}

pub fn levenshtein_costs() -> LevenshteinCosts {
    LevenshteinCosts {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_show_cost() {
        assert_eq!(show_cost(&Insertion(1)), "+1");
        assert_eq!(show_cost(&Deletion(1)), "-1");
        assert_eq!(show_cost(&Substitution(1)), "~1");
        assert_eq!(show_cost(&NoAction(1)), "o1");
    }
    #[test]
    fn test_levenshtein_cost() {
        let lc = levenshtein_costs();
        assert_eq!(lc.insertion_cost(&'a'), 1);
        assert_eq!(lc.deletion_cost(&'a'), 1);
        // no cost for substituting the same letter
        assert_eq!(lc.substitution_cost(&'a', &'a'), 0);
        assert_eq!(lc.substitution_cost(&'a', &'b'), 1);
        assert_eq!(lc.lower_cost(&'a', &'b', 1, 2, 3), Insertion(1));
        assert_eq!(lc.lower_cost(&'a', &'a', 1, 2, 1), Insertion(1));
        assert_eq!(lc.lower_cost(&'a', &'b', 2, 3, 1), Substitution(1));
        assert_eq!(lc.lower_cost(&'a', &'b', 2, 2, 3), Deletion(2));
        assert_eq!(lc.lower_cost(&'a', &'a', 2, 2, 2), Deletion(2));
        assert_eq!(lc.lower_cost(&'a', &'b', 2, 3, 2), Substitution(2));
    }
}
