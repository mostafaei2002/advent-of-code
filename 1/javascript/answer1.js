const { readFileSync } = require("fs");

const q = readFileSync("../question", { encoding: "utf-8" });
const lines = q.split("\n");

function findMatches(line) {
    const matches = line.match(/[0-9]/g);
    const firstMatch = matches.at(0);
    const lastMatch = matches.at(-1);

    console.log(firstMatch, lastMatch);
    return { firstMatch, lastMatch };
}

function solution(lines) {
    let sum = 0;

    lines.forEach((line) => {
        let { firstMatch, lastMatch } = findMatches(line);
        const match = firstMatch + lastMatch;

        sum += Number(match);
    });

    console.log(sum);
    return sum;
}

solution(lines);
