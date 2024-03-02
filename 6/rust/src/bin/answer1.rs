use std::fs;

fn main() {
    let inp = fs::read_to_string("../../../question")
        .unwrap()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .map(|val| val.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();

    let question: Vec<(u64, u64)> = inp[0].clone().into_iter().zip(inp[1].clone()).collect();

    println!("{question:?}");
    println!("Answer is {}", solve(question));
}

fn solve(question: Vec<(u64, u64)>) -> u64 {
    let mut res: u64 = 1;
    for (time, distance) in question {
        let mut cnt: u64 = 0;

        for s in 0..=time {
            if s * (time - s) > distance {
                cnt += 1;
            }
        }

        println!("{cnt}");
        res *= cnt;
    }

    res
}

#[test]
fn test_example() {
    let question = vec![(7, 9), (15, 40), (30, 200)];
    let res = solve(question);

    assert_eq!(res, 288)
}
