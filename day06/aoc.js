const fs = require('fs');

/**
 * Will figure out how many fishes
 * @param {number[]} input Comma-separated string
 * @param {number} days Days in number
 */
const solution = (input, days) => {
    let fishes = new Map();
    for(const i of input) {
        if (!fishes[i]) {
            fishes[i] = 0
        }
        fishes[i]++;
    }
    for(let i = 0; i < days; i++) {
        const newFishes = new Map();
        for(let [key, v] of Object.entries(fishes)) {
            key--;
            if (key == -1) {
                key = 6;
                if (!newFishes[8]) {
                    newFishes[8] = 0;
                }
                newFishes[8] += v;
            }
            if (!newFishes[key]) {
                newFishes[key] = 0;
            }
            newFishes[key] += v;
        }
        fishes = newFishes;
    }
    let sum = 0;
    for(let [_, v] of Object.entries(fishes)) {
        sum += v;
    }
    return sum;
}

(async function() {
    const part = process.env.part || "part1";
    try {
        fs.readFile('./input.txt', (err, buffer) => {
            if (err) {
                throw new Error(err)
            }
            const data = buffer.toString();
            const inputs = data.split(",").map(i => +i);
            const days = part == "part1" ? 80 : 256;
            console.log(`Answer for ${part}: ${solution(inputs, days)}`);
        });
    } catch(fileErr) {
        console.error("Could not load file");
        process.exit(1);
    }
})();