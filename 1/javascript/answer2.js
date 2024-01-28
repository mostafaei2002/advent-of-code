const { readFileSync } = require("fs");

const q = readFileSync("../question", { encoding: "utf-8" });
const lines = q.split("\n");

function findMatches(line) {
    const valueObj = {
        0: 0,
        1: 1,
        2: 2,
        3: 3,
        4: 4,
        5: 5,
        6: 6,
        7: 7,
        8: 8,
        9: 9,
        zero: 0,
        one: 1,
        two: 2,
        three: 3,
        four: 4,
        five: 5,
        six: 6,
        seven: 7,
        eight: 8,
        nine: 9,
    };

    let firstMatch = "";
    let lastMatch = "";

    for (let i = 0; i < line.length; i++) {
        for (let j = i + 1; j < i + 6; j++) {
            const curSlice = line.slice(i, j);
            const cur = valueObj[curSlice];

            if (cur && firstMatch === "") {
                firstMatch = String(cur);
                lastMatch = String(cur);
            } else if (cur) {
                lastMatch = String(cur);
            }
        }
    }
    console.log(line);
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
