from typing import List

import pytest

from codewars.scc import Component, Graph, strongly_connected_components


@pytest.mark.parametrize(
    "graph,expected",
    [
        (
            # Graph from the description
            [[1], [2, 3, 4], [0, 3], [5], [5, 6], [3], [4, 7], [5, 6]],
            [{3, 5}, {4, 6, 7}, {0, 1, 2}],
        ),
        (
            # One more sample
            [[1], [2], [3, 4], [0], [5], [6], [4, 7], []],
            [{7}, {4, 5, 6}, {0, 1, 2, 3}],
        ),
        (
            # Another case
            [[5, 1], [6, 0, 2], [3, 7], [5, 1], [1], [3], [9], [], [6], [3]],
            [{7}, {0, 1, 2, 3, 5, 6, 9}, {4}, {8}],
        ),
        (
            # Empty graph
            [],
            [],
        ),
        (
            # Graph with one vertex
            [[]],
            [{0}],
        ),
        (
            # Contour
            [[1], [2], [3], [4], [5], [0]],
            [{0, 1, 2, 3, 4, 5}],
        ),
    ],
)
def test_scc(graph: Graph, expected: List[Component]) -> None:
    actual = strongly_connected_components(graph)
    assert len(expected) == len(actual)
    assert expected == actual
