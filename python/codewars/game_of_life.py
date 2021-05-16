from dataclasses import dataclass
from typing import List

Grid = List[List[int]]


@dataclass
class LiveZone:
    min_i: int
    min_j: int
    max_i: int
    max_j: int

    @classmethod
    def new(cls, height: int, width: int) -> 'LiveZone':
        return cls(height - 1, width - 1, 0, 0)

    def update(self, i: int, j: int) -> None:
        self.min_i = min(self.min_i, i)
        self.min_j = min(self.min_j, j)
        self.max_i = max(self.max_i, i)
        self.max_j = max(self.max_j, j)


DEAD = 0
LIVE = 1


def inflate(cells: Grid) -> Grid:
    width = len(cells[0])

    grid = [[DEAD] * (width + 2)]
    for row in cells:
        new_row = [DEAD]
        new_row += row
        new_row.append(DEAD)
        grid.append(new_row)
    grid.append([DEAD] * (width + 2))

    return grid


def crop(cells: Grid, zone: LiveZone) -> Grid:
    return [
        [cells[i][j] for j in range(zone.min_j, zone.max_j + 1)]
        for i in range(zone.min_i, zone.max_i + 1)
    ]


def count_alive(i: int, j: int, cells: Grid) -> int:
    height, width = len(cells), len(cells[0])
    return sum(
        cells[x - 1][y - 1]
        for x in range(i - 1, i + 2)
        for y in range(j - 1, j + 2)
        if 1 <= x <= height and 1 <= y <= width and (x, y) != (i, j)
    )


def get_generation(cells: Grid, generations: int) -> Grid:

    for _ in range(generations):
        grid = inflate(cells)
        height, width = len(grid), len(grid[0])
        zone = LiveZone.new(height, width)

        for i in range(height):
            for j in range(width):

                n_alive = count_alive(i, j, cells)

                if grid[i][j] == LIVE:
                    # Rules #1 (underpopulation), #2 (overcrowding) and #3
                    if n_alive < 2 or n_alive > 3:
                        grid[i][j] = DEAD
                else:
                    # Rule #4: dead cell with 3 live neighbours becomes alive
                    if n_alive == 3:
                        grid[i][j] = LIVE

                if grid[i][j] == LIVE:
                    zone.update(i, j)

        cells = crop(grid, zone)

    return cells
