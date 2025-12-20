import assert from "node:assert";

const DIRS8 = [
    [-1, -1], [-1, 0], [-1, 1],
    [0, -1],/* [0, 0], */ [0, 1],
    [1, -1], [1, 0], [1, 1],
];

type CellFn = (row: number, col: number) => unknown;

export function gridWithValues <T>(rows: number, cols: number, value: T): T[][] {
    const grid = new Array(rows);
    for(let i = 0; i < rows; i++) {
        grid[i] = [];
        for(let j = 0; j < cols; j++) {
            grid[i].push(value);
        }
    }
    return grid;
}

export class Grid<T> {
    public readonly rows;
    public readonly cols;

    constructor(private readonly grid: T[][]) {
        this.rows = grid.length;
        this.cols = this.rows > 0 ? grid[0].length : 0;
        assert(grid.every(row => row.length === this.cols));
    }

    at(r: number, c: number): T {
        return this.grid[r][c];
    }

    change(r: number, c: number, val: T) {
        this.grid[r][c] = val;
    }

    inside(r: number, c: number) {
        return 0 <= r && r < this.rows && 0 <= c && c < this.cols;
    }

    forEveryCell(fn: (row: number, col: number) => unknown) {
        for (let i = 0; i < this.rows; i++) {
            for (let j = 0; j < this.cols; j++) {
                fn(i, j);
            }
        }
    }

    * cellIndices(): Generator<[number, number]> {
        for (let i = 0; i < this.rows; i++) {
            for (let j = 0; j < this.cols; j++) {
                yield [i, j];
            }
        }
    }

    * neighbors8(row: number, col: number): Generator<T> {
        for (const [dx, dy] of DIRS8) {
            const [nx, ny] = [row + dx, col + dy];
            if (this.inside(nx, ny)) {
                yield this.grid[nx][ny];
            }
        }
    }

    forEachNeighbor8(row: number, col: number, fn: CellFn) {
        for (const [dx, dy] of DIRS8) {
            const [nx, ny] = [row + dx, col + dy];
            if (this.inside(nx, ny)) {
                fn(nx, ny);
            }
        }
    }

}
