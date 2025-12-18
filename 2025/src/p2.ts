const P1_EXAMPLE = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224," +
    "1698522-1698528,446443-446449,38593856-38593862,565653-565659," +
    "824824821-824824827,2121212118-2121212124";
const P1_INPUT = "874324-1096487,6106748-6273465,1751-4283,294380-348021,5217788-5252660,828815656-828846474,66486-157652,477-1035,20185-55252,17-47,375278481-375470130,141-453,33680490-33821359,88845663-88931344,621298-752726,21764551-21780350,58537958-58673847,9983248-10042949,4457-9048,9292891448-9292952618,4382577-4494092,199525-259728,9934981035-9935011120,6738255458-6738272752,8275916-8338174,1-15,68-128,7366340343-7366538971,82803431-82838224,72410788-72501583";

function calculateBruteForceRepeatTwice(min: number, max: number): number {
    let ans = 0;
    for (let i = 1; i <= 100000; i++) {
        let x = i * (Math.pow(10, Math.ceil(Math.log10(i + 1)))) + i;
        if (min <= x && x <= max) {
            ans += x;
        }
    }
    return ans;
}

function calculateBruteForceRepeatN(min: number, max: number): number {
    let ans = 0;
    let seen = new Set<number>();
    for (let i = 1; i <= 100000; i++) {
        let d = Math.pow(10, Math.ceil(Math.log10(i + 1)));
        let x = i;
        for(let k = 2; x <= max; k++) {
            x = x * d + i;
            if(min <= x && x <= max && !seen.has(x)) {
                seen.add(x);
                ans += x;
            }
        }
    }
    return ans;
}

function solve1(input: string) {
    const inputs = input.split(",");
    let sum = 0;
    for (const input of inputs) {
        const [a, b] = input.split("-").map(s => parseInt(s, 10));
        sum += calculateBruteForceRepeatTwice(a, b);
    }
    return sum;
}

function solve2(input: string) {
    const inputs = input.split(",");
    let sum = 0;
    for (const input of inputs) {
        const [a, b] = input.split("-").map(s => parseInt(s, 10));
        sum += calculateBruteForceRepeatN(a, b);
    }
    return sum;
}

console.log(solve1(P1_INPUT));
console.log(solve1(P1_EXAMPLE));

console.log(solve2(P1_INPUT));
console.log(solve2(P1_EXAMPLE));
