import {FileHandle} from "node:fs/promises";
import {readAllLines, solveAll} from "./lib/utils.js";
import {Grid, gridWithValues} from "./lib/grid.js";
import {Deque} from "./lib/queue.js";


async function solve1(fileHandle: FileHandle) {
    const lines = await readAllLines(fileHandle);
    const charGrid = lines.map(line => [...line]);
    const grid = new Grid(charGrid);
    let cnt = 0;
    grid.forEveryCell((row, col) => {
        if (grid.at(row, col) !== '@') {
            return;
        }
        let paper = 0;
        grid.neighbors8(row, col).forEach(p => {
            paper += p === '@' ? 1 : 0;
        })
        if (paper < 4) {
            cnt += 1;
        }
    })

    console.log(cnt);
}

async function solve2(fileHandle: FileHandle) {
    const lines = await readAllLines(fileHandle);
    const charGrid = lines.map(line => [...line]);
    const grid = new Grid(charGrid);
    const cnt = gridWithValues(grid.rows, grid.cols, 0);

    // console.log(cnt);

    const q = new Deque<[number, number]>();
    grid.forEveryCell((row, col) => {
        if (grid.at(row, col) !== '@') {
            return;
        }
        grid.neighbors8(row, col).forEach(p => {
            cnt[row][col] += p === '@' ? 1 : 0;
        })
        if(cnt[row][col] < 4) {
            q.pushBack([row, col]);
        }
    });

    let ans = 0;
    while(!q.isEmpty()) {
        let [x, y] = q.popFront();
        ans += 1;
        grid.forEachNeighbor8(x, y, (nx, ny) => {
            if(cnt[nx][ny] >= 4) {
                cnt[nx][ny] -= 1;
                if(cnt[nx][ny] < 4) {
                    q.pushBack([nx, ny]);
                }
            }
        })
    }
    console.log(ans);
}

await solveAll(4, [solve1, solve2]);
