use std::fs;

fn main() {
    let question: Vec<u64> = fs::read_to_string("../../../question")
        .unwrap()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .collect::<Vec<&str>>()
                .join("")
        })
        .map(|val| val.parse::<u64>().unwrap())
        .collect();

    println!("Answer is {}", solve(question[0], question[1]));
}

fn solve(time: u64, distance: u64) -> u128 {
    let mut res: u128 = 0;
    println!("{time}, {distance}");

    for s in 0..time {
        if s * (time - s) > distance {
            res += 1;
        }
    }

    res
}

#[test]
fn test_example() {
    let time: u64 = 71530;
    let distance: u64 = 940200;
    let res = solve(time, distance);

    assert_eq!(res, 71503)
}
