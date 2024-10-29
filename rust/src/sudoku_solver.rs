use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

const N: usize = 9;

type Board = [[u8; N]; N];
type Var = (usize, usize);
type Val = u8;

/// Solves given sudoku puzzle in place
pub fn sudoku(puzzle: &mut Board) {
    let csp = CSP::new(puzzle);

    let mut remaining = csp
        .domains
        .iter()
        .filter_map(|(var, domain)| if domain.len() > 1 { Some(var) } else { None })
        .collect();

    let mut assignment = Assignment(puzzle);
    let mut inference = Ac3::new(csp.constraints);

    // Solve Sudoku CSP
    let feasible = solve(
        &mut assignment,
        &mut remaining,
        &csp.domains,
        &mut inference,
    );

    assert!(feasible, "no feasible solution found");
}

#[derive(Debug)]
#[repr(transparent)]
struct Assignment<'a>(&'a mut Board);

impl Assignment<'_> {
    #[inline]
    fn assign(&mut self, (row, col): Var, val: Val) {
        self.0[row][col] = val;
    }

    #[inline]
    fn unassign(&mut self, (row, col): Var) {
        self.0[row][col] = 0;
    }

    fn consistent(&self, var: Var, val: Val) -> bool {
        match self[var] {
            0 => true,
            v => v != val,
        }
    }
}

impl std::ops::Index<Var> for Assignment<'_> {
    type Output = u8;

    #[inline]
    fn index(&self, (row, col): Var) -> &Self::Output {
        &self.0[row][col]
    }
}

#[derive(Clone, Debug)]
struct Domain {
    vals: [bool; N],
    size: usize,
}

impl Domain {
    fn new<R>(range: R) -> Self
    where
        R: std::ops::RangeBounds<Val> + Iterator<Item = Val>,
    {
        let mut vals = [false; N];

        for val in range {
            vals[val as usize - 1] = true;
        }

        Self {
            vals,
            size: vals.len(),
        }
    }

    #[inline]
    fn singleton(val: Val) -> Self {
        Self::new(val..=val)
    }

    #[inline]
    fn len(&self) -> usize {
        self.size
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn iter(&self) -> impl Iterator<Item = Val> + '_ {
        self.vals
            .iter()
            .enumerate()
            .filter_map(|(v, &present)| if present { Some(v as u8 + 1) } else { None })
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = (&'_ mut bool, Val)> + '_ {
        self.vals.iter_mut().enumerate().filter_map(|(v, slot)| {
            if *slot {
                Some((slot, v as u8 + 1))
            } else {
                None
            }
        })
    }

    fn assign(&mut self, val: Val) {
        self.vals.fill(false);
        self.vals[val as usize - 1] = true;
        self.size = 1;
    }

    /// Procedure that deletes any value from `self` which is inconsistent with values from `other`
    /// under constraints `x != y`.
    fn revise(&mut self, other: &Domain) -> bool {
        let mut deleted = 0;

        for (x_slot, x_val) in self.iter_mut() {
            // If there's no value for y that statisfies x != y
            if other.iter().all(|y_val| x_val == y_val) {
                // remove x
                *x_slot = false;
                deleted += 1;
            }
        }

        self.size -= deleted;
        deleted > 0
    }
}

impl Default for Domain {
    fn default() -> Self {
        Self {
            vals: [true; N],
            size: N,
        }
    }
}

#[derive(Clone, Debug, Default)]
#[repr(transparent)]
struct DomainSet([[Domain; N]; N]);

impl DomainSet {
    #[inline]
    fn assign(&mut self, (row, col): Var, val: Val) {
        self.0[row][col].assign(val)
    }

    fn get_pair_mut(
        &mut self,
        (x_row, x_col): Var,
        (y_row, y_col): Var,
    ) -> Result<(&mut Domain, &mut Domain), &mut Domain> {
        if x_row == y_row {
            split_at2_mut(&mut self.0[x_row], x_col, y_col)
        } else {
            let Ok((with_x, with_y)) = split_at2_mut(&mut self.0, x_row, y_row) else {
                unreachable!("same row checked above");
            };
            Ok((&mut with_x[x_col], &mut with_y[y_col]))
        }
    }

    fn iter(&self) -> impl Iterator<Item = (Var, &Domain)> + '_ {
        self.0
            .iter()
            .enumerate()
            .flat_map(|(row, ds)| ds.iter().enumerate().map(move |(col, d)| ((row, col), d)))
    }
}

impl std::ops::Index<Var> for DomainSet {
    type Output = Domain;

    #[inline]
    fn index(&self, (row, col): Var) -> &Self::Output {
        &self.0[row][col]
    }
}

impl std::ops::IndexMut<Var> for DomainSet {
    #[inline]
    fn index_mut(&mut self, (row, col): Var) -> &mut Self::Output {
        &mut self.0[row][col]
    }
}

impl From<&Board> for DomainSet {
    fn from(board: &Board) -> Self {
        let mut domains = Self::default();
        for (row, vals) in board.iter().enumerate() {
            for (col, &val) in vals.iter().enumerate().filter(|(_, &val)| val > 0) {
                domains[(row, col)] = Domain::singleton(val);
            }
        }
        domains
    }
}

fn split_at2_mut<T>(slice: &mut [T], x: usize, y: usize) -> Result<(&mut T, &mut T), &mut T> {
    use std::cmp::Ordering::*;
    match x.cmp(&y) {
        Equal => Err(&mut slice[x]),
        Greater => split_at2_mut(slice, y, x).map(|(y, x)| (x, y)),
        Less => {
            let (_, slice) = slice.split_at_mut(x);
            let ([at_x], slice) = slice.split_at_mut(1) else {
                unreachable!()
            };
            let (_, [at_y, ..]) = slice.split_at_mut(y - x - 1) else {
                unreachable!()
            };
            Ok((at_x, at_y))
        }
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
struct CSP {
    domains: DomainSet,
    constraints: HashMap<Var, HashSet<Var>>,
}

impl CSP {
    fn new(puzzle: &Board) -> Self {
        let domains = DomainSet::from(puzzle);

        // Set of binary constraints between variables
        //   {x: {y}, y: {x}} iff x != y in constraints
        let mut constraints: HashMap<Var, HashSet<Var>> = HashMap::new();

        // TODO: global alldiff constraint
        for i in 0..N {
            // Values in each row must all be different
            for (x, y) in (0..N).map(|col| (i, col)).tuple_combinations() {
                constraints.entry(x).or_default().insert(y);
                constraints.entry(y).or_default().insert(x);
            }

            // Values in each column must all be different
            for (x, y) in (0..N).map(|row| (row, i)).tuple_combinations() {
                constraints.entry(x).or_default().insert(y);
                constraints.entry(y).or_default().insert(x);
            }
        }

        // Values in each 3x3 square must all be different
        for (i, j) in (0..N).step_by(3).cartesian_product((0..N).step_by(3)) {
            let block = (0..3)
                .cartesian_product(0..3)
                .map(|(row, col)| (i + row, j + col));

            for (x, y) in block.tuple_combinations() {
                constraints.entry(x).or_default().insert(y);
                constraints.entry(y).or_default().insert(x);
            }
        }

        Self {
            domains,
            constraints,
        }
    }
}

trait Inference {
    fn arcs(&self, var: Var) -> &HashSet<Var>;

    fn infer(
        &mut self,
        var: Var,
        val: Val,
        domains: &DomainSet,
        remaining: &HashSet<Var>,
    ) -> Option<DomainSet>;
}

struct Ac3 {
    queue: VecDeque<(Var, Var)>,
    constraints: HashMap<Var, HashSet<Var>>,
}

impl Ac3 {
    #[inline]
    fn new(constraints: HashMap<Var, HashSet<Var>>) -> Self {
        Self {
            queue: VecDeque::new(),
            constraints,
        }
    }
}

impl Inference for Ac3 {
    #[inline]
    fn arcs(&self, var: Var) -> &HashSet<Var> {
        &self.constraints[&var]
    }

    fn infer(
        &mut self,
        var: Var,
        val: Val,
        domains: &DomainSet,
        remaining: &HashSet<Var>,
    ) -> Option<DomainSet> {
        self.queue.clear();

        // TODO: look into some ways to optimize around this clone
        // Restrict domains with new assignment var := val
        let mut domains = domains.clone();
        domains.assign(var, val);

        // For maintaining AC it's enough to consider remaining neighbors of var
        let arcs = self.constraints[&var].iter().filter_map(|x| {
            if remaining.contains(x) {
                Some((*x, var))
            } else {
                None
            }
        });

        self.queue.extend(arcs);

        while let Some((x, y)) = self.queue.pop_front() {
            // x != y is the only possible constraint
            let (dx, dy) = domains.get_pair_mut(x, y).expect("domain for x and y");

            let cx = &self.constraints[&x];

            if cx.contains(&y) && dx.revise(dy) {
                if dx.is_empty() {
                    return None;
                }
                // Add arcs (z, x) for all constraints {x, z} for z other than y
                self.queue.extend(
                    cx.iter()
                        .filter_map(|&z| if z != y { Some((z, x)) } else { None }),
                );
            }
        }

        Some(domains)
    }
}

fn solve(
    assignment: &mut Assignment<'_>,
    remaining: &mut HashSet<Var>,
    domains: &DomainSet,
    inference: &mut dyn Inference,
) -> bool {
    // Min. remaining value selection
    let Some(&var) = remaining.iter().min_by_key(|v| domains[**v].len()) else {
        // No remaining variables to assign, solution has been found
        return true;
    };

    remaining.remove(&var);

    for val in domains[var].iter() {
        // Check if assignment var := val is consistent
        if inference
            .arcs(var)
            .iter()
            .all(|&x| assignment.consistent(x, val))
        {
            assignment.assign(var, val);

            // Infer feasible domains that are arc-consistent using AC3 and check if the inference
            // found this sub-space feasible
            if let Some(revised_domains) = inference.infer(var, val, domains, remaining) {
                if solve(assignment, remaining, &revised_domains, inference) {
                    return true;
                }
            }

            assignment.unassign(var);
        }
    }

    remaining.insert(var);
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    type Sudoku = [[u8; N]; N];

    struct TestCase {
        instance: Sudoku,
        solution: Sudoku,
    }

    #[fixture]
    fn puzzle_1() -> TestCase {
        TestCase {
            instance: [
                [6, 0, 5, 7, 2, 0, 0, 3, 9],
                [4, 0, 0, 0, 0, 5, 1, 0, 0],
                [0, 2, 0, 1, 0, 0, 0, 0, 4],
                [0, 9, 0, 0, 3, 0, 7, 0, 6],
                [1, 0, 0, 8, 0, 9, 0, 0, 5],
                [2, 0, 4, 0, 5, 0, 0, 8, 0],
                [8, 0, 0, 0, 0, 3, 0, 2, 0],
                [0, 0, 2, 9, 0, 0, 0, 0, 1],
                [3, 5, 0, 0, 6, 7, 4, 0, 8],
            ],
            solution: [
                [6, 1, 5, 7, 2, 4, 8, 3, 9],
                [4, 8, 7, 3, 9, 5, 1, 6, 2],
                [9, 2, 3, 1, 8, 6, 5, 7, 4],
                [5, 9, 8, 4, 3, 2, 7, 1, 6],
                [1, 3, 6, 8, 7, 9, 2, 4, 5],
                [2, 7, 4, 6, 5, 1, 9, 8, 3],
                [8, 4, 9, 5, 1, 3, 6, 2, 7],
                [7, 6, 2, 9, 4, 8, 3, 5, 1],
                [3, 5, 1, 2, 6, 7, 4, 9, 8],
            ],
        }
    }

    #[fixture]
    fn puzzle_2() -> TestCase {
        TestCase {
            instance: [
                [0, 0, 8, 0, 3, 0, 5, 4, 0],
                [3, 0, 0, 4, 0, 7, 9, 0, 0],
                [4, 1, 0, 0, 0, 8, 0, 0, 2],
                [0, 4, 3, 5, 0, 2, 0, 6, 0],
                [5, 0, 0, 0, 0, 0, 0, 0, 8],
                [0, 6, 0, 3, 0, 9, 4, 1, 0],
                [1, 0, 0, 8, 0, 0, 0, 2, 7],
                [0, 0, 5, 6, 0, 3, 0, 0, 4],
                [0, 2, 9, 0, 7, 0, 8, 0, 0],
            ],
            solution: [
                [9, 7, 8, 2, 3, 1, 5, 4, 6],
                [3, 5, 2, 4, 6, 7, 9, 8, 1],
                [4, 1, 6, 9, 5, 8, 3, 7, 2],
                [8, 4, 3, 5, 1, 2, 7, 6, 9],
                [5, 9, 1, 7, 4, 6, 2, 3, 8],
                [2, 6, 7, 3, 8, 9, 4, 1, 5],
                [1, 3, 4, 8, 9, 5, 6, 2, 7],
                [7, 8, 5, 6, 2, 3, 1, 9, 4],
                [6, 2, 9, 1, 7, 4, 8, 5, 3],
            ],
        }
    }

    #[rstest]
    fn solve_puzzle_1(mut puzzle_1: TestCase) {
        sudoku(&mut puzzle_1.instance);
        assert_eq!(puzzle_1.instance, puzzle_1.solution);
    }

    #[rstest]
    fn solve_puzzle_2(mut puzzle_2: TestCase) {
        sudoku(&mut puzzle_2.instance);
        assert_eq!(puzzle_2.instance, puzzle_2.solution);
    }
}
