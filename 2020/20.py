import copy
import math
import re
from typing import Self


def parse_tile_id(s: str) -> int:
    match = re.search(r"\d+", s)
    if match:
        return int(match[0])
    else:
        raise ValueError("Error parsing {}", s)

GRID_SIZE = 10

class Tile:
    BUFFER = []

    def __init__(self, id: int, grid: list[list[str]]):
        self.id = id
        self.grid = grid
    def grid_str(self):
        return "\n".join(["".join(row) for row in self.grid])

    def __repr__(self):
        return f"id={self.id}\ngrid={self.grid_str()}\n"

    def match_right(self, other: Self) -> bool:
        for i in range(len(self.grid)):
            if self.grid[i][-1] != other.grid[i][0]:
                return False
        return True

    def match_bot(self, other: Self) -> bool:
        for i in range(len(self.grid)):
            if self.grid[-1][i] != other.grid[0][i]:
                return False
        return True

    def rotate(self):
        new_grid = copy.deepcopy(self.grid)
        n = len(self.grid)
        for i in range(n):
            for j in range(n):
                new_grid[j][n-1-i] = self.grid[i][j]
        self.grid = new_grid

    def flip(self):
        self.grid.reverse()



def solve(input_str: str):
    lines = input_str.splitlines()
    tiles = []
    lines_per_tile = 12
    n_tiles = len(lines) // lines_per_tile

    for i in range(n_tiles):
        s = i * lines_per_tile
        id = parse_tile_id(lines[s])
        tiles.append(Tile(id, [list(row) for row in lines[s+1:s+11]]))

    n = round(math.sqrt(n_tiles))
    print(n, n_tiles)


    out = [[-1] * n for _ in range(n)]
    seen = set()

    # for _ in range(2): # Flips
    #     for _ in range(4): # Rotations
    #         print(tiles[0].grid_str(), end="\n\n")
    #         tiles[0].rotate()
    #     tiles[0].flip()

    def tiles_to_str():
        s = []
        for i in range(n * GRID_SIZE):
            for j in range(n * GRID_SIZE):
                k = out[i//GRID_SIZE][j//GRID_SIZE]
                if k == -1:
                    s.append("X")
                else:
                    s.append(tiles[k].grid[i % GRID_SIZE][j % GRID_SIZE])
            s.append("\n")
        return "".join(s)

    def rec(row, col) -> bool:
        # print(row, col, out)
        # print(tiles_to_str())
        if row == n:
            return True
        if col + 1 == n:
            nrow, ncol = row + 1, 0
        else:
            nrow, ncol = row, col + 1
        for i in range(len(tiles)):
            if i in seen:
                continue
            for _ in range(2): # Flips
                for _ in range(4): # Rotations
                    matches_existing = True
                    if row > 0:
                        matches_existing &= tiles[out[row-1][col]].match_bot(tiles[i])
                    if col > 0:
                        matches_existing &= tiles[out[row][col-1]].match_right(tiles[i])
                    if matches_existing:
                        seen.add(i)
                        out[row][col] = i
                        if rec(nrow, ncol):
                            return True
                        out[row][col] = -1
                        seen.remove(i)

                    tiles[i].rotate()
                tiles[i].flip()
        return False

    if not rec(0, 0):
        print("Did not find a solution...")
        return False


    ids: list[int] = list(map(lambda rc: tiles[out[rc[0]][rc[1]]].id, [(0, n-1), (0, 0), (n-1, 0), (n-1, n-1)]))

    print(ids[0] * ids[1] * ids[2] * ids[3])



with open("../inputs/y20/20.1.in", "r") as f:
    solve(f.read())
with open("../inputs/y20/20.in", "r") as f:
    solve(f.read())

