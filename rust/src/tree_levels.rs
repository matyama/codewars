use std::collections::VecDeque;

#[derive(Debug)]
pub struct Node {
    value: u32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    #[inline]
    pub fn new(value: u32) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }

    pub fn left(mut self, left: Self) -> Self {
        self.left.replace(Box::new(left));
        self
    }

    pub fn right(mut self, right: Self) -> Self {
        self.right.replace(Box::new(right));
        self
    }
}

/// Traverses given tree in BFS order yielding its values
pub fn tree_by_levels(root: &Node) -> Vec<u32> {
    let mut values = vec![];

    let mut queue = VecDeque::new();
    queue.push_back(root);

    while let Some(n) = queue.pop_front() {
        values.push(n.value);

        if let Some(left) = &n.left {
            queue.push_back(left);
        }

        if let Some(right) = &n.right {
            queue.push_back(right);
        }
    }

    values
}

#[cfg(test)]
mod sample_tests {
    use super::*;

    fn test_tree(root: &Node, expected: &[u32]) {
        assert_eq!(
            tree_by_levels(root),
            expected,
            "\nYour result (left) didn't match the expected output (right)."
        );
    }

    #[test]
    fn root_only() {
        test_tree(&Node::new(42), &[42]);
    }

    #[test]
    fn complete_tree() {
        let root = Node::new(1)
            .left(Node::new(2).left(Node::new(4)).right(Node::new(5)))
            .right(Node::new(3).left(Node::new(6)));

        test_tree(&root, &[1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn example1() {
        let root = Node::new(2)
            .left(Node::new(8).left(Node::new(1)).right(Node::new(3)))
            .right(Node::new(9).left(Node::new(4)).right(Node::new(5)));

        test_tree(&root, &[2, 8, 9, 1, 3, 4, 5]);
    }

    #[test]
    fn example2() {
        let root = Node::new(1)
            .left(Node::new(8).right(Node::new(3)))
            .right(Node::new(4).right(Node::new(5).right(Node::new(7))));

        test_tree(&root, &[1, 8, 4, 3, 5, 7]);
    }
}
