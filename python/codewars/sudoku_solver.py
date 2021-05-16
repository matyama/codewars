from collections import defaultdict, deque
from itertools import combinations, product
from typing import Dict, List, Optional, Set, Tuple

Variable = Tuple[int, int]
Domain = Set[int]
Constraint = Tuple[Variable]


# pylint: disable=too-many-locals
def sudoku(puzzle: List[List[int]]) -> List[List[int]]:
    N = 9

    # Domains for all variables (positions on the board)
    domains = {
        (row, col): {val} if val > 0 else set(range(1, N + 1))
        for row, vals in enumerate(puzzle)
        for col, val in enumerate(vals)
    }

    # Set of binary constraints between variables
    #   {x: {y}, y: {x}} iff x != y in constraints
    constraints = defaultdict(set)

    for i in range(N):

        # Values in each row must all be different
        row_vars = ((i, col) for col in range(N))
        for x, y in combinations(row_vars, 2):
            constraints[x].add(y)
            constraints[y].add(x)

        # Values in each column must all be different
        col_vars = ((row, i) for row in range(N))
        for x, y in combinations(col_vars, 2):
            constraints[x].add(y)
            constraints[y].add(x)

    # Values in each 3x3 square must all be different
    for i, j in product(range(0, N, 3), repeat=2):
        block = ((i + row, j + col) for row in range(3) for col in range(3))
        for x, y in combinations(block, 2):
            constraints[x].add(y)
            constraints[y].add(x)

    def revise(domain_x: Domain, domain_y: Domain) -> bool:
        deleted = False
        for x_val in list(domain_x):
            # If there's no value for y that statisfies x != y
            if all(x_val == y_val for y_val in domain_y):
                domain_x.remove(x_val)
                deleted = True
        return deleted

    def ac3(
        var: Variable,
        val: int,
        domains: Dict[Variable, Domain],
        remaining: Set[Variable],
    ) -> Optional[Dict[Variable, Domain]]:

        domains = {x: set(dx) for x, dx in domains.items()}
        domains[var] = {val}

        # For maintaining AC it's enough to consider remaining neighbors of var
        queue = deque((x, var) for x in constraints[var] if x in remaining)

        while queue:
            x, y = queue.popleft()
            # x != y is the only possible constraint
            if y in constraints[x] and revise(domains[x], domains[y]):
                if not domains[x]:
                    return None
                # Add arcs (z, x) for all constraints {x, z} for z other than y
                queue.extend((z, x) for z in constraints[x] if z != y)

        return domains

    def solve(
        assignment: Dict[Variable, int],
        remaining: Set[Variable],
        domains: Dict[Variable, Domain],
    ) -> Optional[Dict[Variable, int]]:

        if not remaining:
            return assignment

        # Min. remaining value selection
        var = min(remaining, key=lambda v: len(domains[v]))
        remaining.remove(var)

        for val in domains[var]:

            # Check if assignment var := val is consistent
            if all(val != assignment.get(x) for x in constraints[var]):
                assignment[var] = val

                # Infer feasible domains that are arc-consistent using AC3
                revised_domains = ac3(var, val, domains, remaining)

                # Check if the inference found this sub-space feasible
                if revised_domains is not None:
                    solution = solve(assignment, remaining, revised_domains)
                    if solution is not None:
                        return solution

                del assignment[var]

        remaining.add(var)
        return None

    # Initial assignment
    assignment = {
        var: next(iter(domain))
        for var, domain in domains.items()
        if len(domain) == 1
    }

    # Solve Sudoku CSP
    solution = solve(
        assignment=assignment,
        remaining=domains.keys() - assignment.keys(),
        domains=domains,
    )
    assert solution is not None, 'failed'

    # Fill the board from the final solution
    for (row, col), val in solution.items():
        puzzle[row][col] = val

    return puzzle
