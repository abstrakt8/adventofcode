import {FileHandle, open,} from "node:fs/promises";
import {join, resolve} from "node:path";

type Solver = (file: FileHandle) => Promise<unknown>;

export async function solveAll(dayNumber: number, solvers: Solver[], suffixes = [".example", ""]) {

    for (const [idx, solver] of solvers.entries()) {
        console.log(`Solver ${idx + 1}`);
        for (const suffix of suffixes) {
            const inputFileName = `p${dayNumber}${suffix}.in`;
            const inputFilePath = resolve(join(import.meta.dirname, `../../examples/${inputFileName}`));

            console.log("Input:", inputFileName);
            try {
                const input = await open(inputFilePath);
                await solver(input);
            } catch (err: any) {
                if (err.code === 'ENOENT') {
                    console.warn(`Skipped: ${inputFileName} (File does not exist)`);
                } else {
                    console.error("Some other error", err);
                }
            }
        }
    }
}

export async function readAllLines(fileHandle: FileHandle) {
    const lines = [];
    for await (const line of fileHandle.readLines()) {
        if (line.trim()) {
            lines.push(line);
        }
    }
    return lines;
}
