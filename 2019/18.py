# BFS: From one place to the other can change depending on which cells were unblocked.
# 26! permutations unfeasible
# 80x80 chars -> 6400 positions, but most irrelevant
# 26x2 + 1 (@) positions relevant => 53
# The large doors aren't even relevant
# 26 + 1 => 27 positions
import heapq
from collections import deque, defaultdict
from typing import DefaultDict

class State:
    def __init__(self, dist: int, pos: str, mask: int):
        self.mask = mask
        self.dist = dist
        self.pos = pos


def alpha_idx(c: str):
    if not c.isalpha():
        return None
    return ord(c.lower()) - ord('a')


def is_bit_set(mask: int, i) -> bool:
    return (mask >> i) & 1


def solve(input_file: str):
    grid = [list(s) for s in input_file.splitlines()]
    n, m = len(grid), len(grid[0])

    def modify():
        for i in range(n):
            for j in range(m):
                if grid[i][j] == "@":
                    for x in [-1, 0, 1]:
                        for y in [-1, 0, 1]:
                            grid[i+x][j+y] = '#'
                    grid[i-1][j-1] = '@'
                    grid[i-1][j+1] = '!'
                    grid[i+1][j-1] = '%'
                    grid[i+1][j+1] = '$'
                    return


    def neighbors(x, y):
        for (dx, dy) in [(0, 1), (0, -1), (-1, 0), (1, 0)]:
            nx, ny = x + dx, y + dy
            if 0 <= nx < n and 0 <= ny < m:
                yield nx, ny

    def is_entrance(c: str):
        return c in "@!$%"
    def key_field(c: str):
        return c.isalpha() or is_entrance(c)

    modify()

    dist: DefaultDict[str, DefaultDict[str, int]] = defaultdict(lambda: defaultdict(int))
    keys = set()
    for (i, row) in enumerate(grid):
        for (j, cell) in enumerate(row):
            if not key_field(cell):
                continue
            if not is_entrance(cell) and cell.islower():
                keys.add(cell)
            q = deque([(i, j)])
            seen = {(i, j)}
            steps = 0
            while q:
                for _ in range(len(q)):
                    (x, y) = q.popleft()
                    if key_field(grid[x][y]):
                        dist[cell][grid[x][y]] = steps
                        if grid[x][y] != cell:
                            continue
                    for (nx, ny) in neighbors(x, y):
                        if grid[nx][ny] != '#' and (nx, ny) not in seen:
                            q.append((nx, ny))
                            seen.add((nx, ny))
                steps += 1

    for row in grid:
        print("".join(row))

    pq = [(0, ('@', '!', '$', '%'), 0)]
    cost = defaultdict(lambda: 10 ** 9)
    cost[(('@', '!', '$', '%'), 0)] = 0
    while pq:
        (cur_cost, cur_poss, cur_mask) = heapq.heappop(pq)
        if cur_mask.bit_count() == len(keys):
            return cur_cost

        if cur_cost != cost[(cur_poss, cur_mask)]:
            continue
        for who in range(len(cur_poss)):
            cur_pos = cur_poss[who]
            for new_pos in dist[cur_pos]:
                vi = alpha_idx(new_pos)
                if not (is_entrance(new_pos) or new_pos.islower() or is_bit_set(cur_mask, vi)):
                    continue

                new_poss: tuple[str, str, str, str] = cur_poss[:who] + (new_pos, ) + cur_poss[who+1:]
                new_cost = cur_cost + dist[cur_pos][new_pos]
                new_mask = cur_mask
                if vi is not None:
                    new_mask |= (1 << vi)

                if new_cost < cost[(new_poss, new_mask)]:
                    cost[(new_poss, new_mask)] = new_cost
                    heapq.heappush(pq, (new_cost, new_poss, new_mask))
    return -1

prefix = "../inputs/y19/",
# inputs = [("18.1.in", 132), ("18.2.in", 136), ("18.3.in", 81), ("18.in", None)]
inputs = [("18.4.in", 8), ("18.in", None)]
for (file, expected) in inputs:
    with open("../inputs/y19/" + file, 'r') as f:
        print(solve(f.read()), expected)
