import {FileHandle, open} from "node:fs/promises";
import {join, resolve} from "node:path";

function solveDP(s: string, k: number) {
    let dp: number[] = new Array(k + 1);
    dp.fill(0);

    for (let i = 0; i < s.length; i++) {
        const x = parseInt(s[i], 10);
        for (let j = dp.length - 1; j >= 1; j--) {
            dp[j] = Math.max(dp[j], dp[j - 1] * 10 + x);
        }
    }
    return dp[dp.length - 1];
}

async function solve1(file: FileHandle) {
    let ans = 0;
    for await (const line of file.readLines()) {
        if (!line.trim()) {
            continue;
        }
        ans += solveDP(line, 2);
    }
    return ans;
}

async function solve2(file: FileHandle) {
    let ans = 0;
    for await (const line of file.readLines()) {
        if (!line.trim()) {
            continue;
        }
        ans += solveDP(line, 12);
    }
    return ans;
}

[".example", ""].forEach(async suffix => {
    for (const solver of [solve1, solve2]) {
        let input = await open(resolve(join(import.meta.dirname, `../examples/p3${suffix}.in`)));
        console.log(await solver(input));
    }
});

