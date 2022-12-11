from codewars.tree_by_levels import Node, tree_by_levels


def test_empty() -> None:
    assert not tree_by_levels(None)


def test_tree() -> None:
    tree = Node(
        Node(None, Node(None, None, 4), 2),
        Node(Node(None, None, 5), Node(None, None, 6), 3),
        1,
    )
    assert tree_by_levels(tree) == [1, 2, 3, 4, 5, 6]


def test_example1() -> None:
    tree = Node(
        Node(Node(None, None, 1), Node(None, None, 3), 8),
        Node(Node(None, None, 4), Node(None, None, 5), 9),
        2,
    )
    assert tree_by_levels(tree) == [2, 8, 9, 1, 3, 4, 5]


def test_example2() -> None:
    tree = Node(
        Node(None, Node(None, None, 3), 8),
        Node(None, Node(None, Node(None, None, 7), 5), 4),
        1,
    )
    assert tree_by_levels(tree) == [1, 8, 4, 3, 5, 7]
