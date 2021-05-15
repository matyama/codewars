from typing import List, Sequence


def eliminate(ranks: Sequence[int]) -> List[int]:
    """
    Pairwise eliminate contestants.

    If there's odd number of ranks, the last one
    is propagated as the first one in the result.
    """

    n = len(ranks)
    last = n if n % 2 == 0 else n - 1

    remaining = [max(ranks[i], ranks[i + 1]) for i in range(0, last, 2)]

    if n % 2 == 1:
        remaining.insert(0, ranks[-1])

    return remaining


def tourney(inp: Sequence[int]) -> List[List[int]]:

    prev = 0
    rounds = [list(inp)]

    # Play rounds until there's just one winner
    while len(rounds[prev]) > 1:
        rounds.append(eliminate(rounds[prev]))
        prev += 1

    return rounds
