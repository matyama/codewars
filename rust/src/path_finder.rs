use std::{
    collections::{BinaryHeap, HashSet},
    ops::Deref,
    str::FromStr,
};

use strum::{EnumIter, IntoEnumIterator};

/// Represents a position in the maze: (row, col)
type State = (usize, usize);

#[derive(Debug, EnumIter)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
enum Cell {
    Empty(u32),
    Wall,
}

#[repr(transparent)]
struct Maze(Vec<Vec<Cell>>);

impl Deref for Maze {
    type Target = [Vec<Cell>];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Maze {
    /// Estimate an optimistic distance to the exit.
    ///
    /// Computes the *Manhattan distance* (*consistent heuristic*) between state `(r, c)` and the
    /// exit state `(n - 1, n - 1)`.
    ///  
    #[inline]
    fn exit_dist(r: usize, c: usize, n: usize) -> u32 {
        (n - 1 - r + n - 1 - c) as u32
    }

    #[inline]
    fn empty(&self, row: usize, col: usize) -> bool {
        matches!(self.0[row][col], Cell::Empty(_))
    }

    fn go(&self, state: State, direction: Direction) -> Option<State> {
        let n = self.len();
        let (r, c) = state;
        match direction {
            Direction::North if r >= 1 && self.empty(r - 1, c) => Some((r - 1, c)),
            Direction::East if c + 1 < n && self.empty(r, c + 1) => Some((r, c + 1)),
            Direction::South if r + 1 < n && self.empty(r + 1, c) => Some((r + 1, c)),
            Direction::West if c >= 1 && self.empty(r, c - 1) => Some((r, c - 1)),
            _ => None,
        }
    }

    #[inline]
    fn h(&self, state: State) -> u32 {
        if let Cell::Empty(h) = self[state.0][state.1] {
            h
        } else {
            u32::MAX
        }
    }
}

impl FromStr for Maze {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split('\n')
            .enumerate()
            .map(|(r, row)| {
                let n = row.len();
                row.chars()
                    .enumerate()
                    .map(|(c, value)| match value {
                        '.' => Ok(Cell::Empty(Self::exit_dist(r, c, n))),
                        'W' => Ok(Cell::Wall),
                        _ => Err(format!("invalid cell '{}'", value)),
                    })
                    .collect()
            })
            .collect::<Result<_, _>>()
            .map(Maze)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Node {
    state: State,
    g: u32,
    h: u32,
}

impl Node {
    #[inline]
    fn init(maze: &Maze) -> Self {
        let state = (0, 0);
        Self {
            state,
            g: 0,
            h: maze.h(state),
        }
    }

    fn successors<'a>(&'a self, maze: &'a Maze) -> impl Iterator<Item = Self> + 'a {
        Direction::iter()
            .filter_map(|direction| maze.go(self.state, direction))
            .map(|state| Self {
                state,
                g: self.g + 1,
                h: maze.h(state),
            })
    }

    #[inline]
    fn f<const W: u32>(&self) -> u32 {
        self.g + W * self.h
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.f::<0>().cmp(&other.f::<0>()).reverse()
    }
}

/// Checks for a start-to-exit path in a maze NxN where:
///  - `[0, 0]` is the start and `[N-1, N-1]` is the exit position (both always empty)
///  - positions are either empty (`'.'`) or walls (`'W'`)
///  - there are 4 directions: N, E, S, W
///
///  Implemnted as an A* search with Manhattan distance heuristic.
pub fn path_finder(maze: &str) -> bool {
    let maze = maze.parse().unwrap();

    let mut open = BinaryHeap::new();
    let mut closed = HashSet::new();

    open.push(Node::init(&maze));

    while let Some(node) = open.pop() {
        // h is goal aware
        if node.h == 0 {
            return true;
        }

        if closed.contains(&node.state) {
            continue;
        }

        closed.insert(node.state);

        for succ in node.successors(&maze) {
            if !closed.contains(&succ.state) {
                open.push(succ);
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case::short_path(
        "\
        .W.\n\
        .W.\n\
        ...\
        ",
        true
    )]
    #[case::medium_path(
        "\
        ......\n\
        ......\n\
        ......\n\
        ......\n\
        ......\n\
        ......\
        ",
        true
    )]
    #[case::edges(
        "\
        ...\n\
        W.W\n\
        ...\
        ",
        true
    )]
    #[case::random_small(
        "\
        ...WW.W\n\
        .......\n\
        WW.W...\n\
        ..W..WW\n\
        ...W..W\n\
        .......\n\
        .W.W.W.\
        ",
        true
    )]
    #[case::no_path(
        "\
        ......\n\
        ......\n\
        ......\n\
        ......\n\
        .....W\n\
        ....W.\
        ",
        false
    )]
    fn test_maze(#[case] maze: &str, #[case] expect: bool) {
        let actual = path_finder(maze);

        assert!(
            actual == expect,
            "Test failed!\n\
             Got:      {}\n\
             Expected: {}\n\
             Maze was: \n\
             {}\n",
            actual,
            expect,
            maze
        );
    }
}
