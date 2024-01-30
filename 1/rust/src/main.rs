use std::{collections::HashMap, fs};


fn find_matches(line: &str) -> String {
    let le: usize = line.len();
    let mut first_match: &str = "";
    let mut last_match: &str = "";

    for i in 0..le {
        let cur = &line[i..i+1];
        match cur {
            "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
                if first_match == "" {
                    first_match = cur;
                }
                last_match = cur;
            },
            _ => continue
        }
    };
    return first_match.to_owned() + last_match
}

fn find_matches2(line: &str) -> String {
    let valuemap = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9")
    ]);

    let le: usize = line.len();
    let mut first_match: &str = "";
    let mut last_match: &str = "";

    for i in 0..le {
        for j in i+1..i+6 {
            if j > le {
                continue
            }

            let cur = &line[i..j];
            // println!("{cur}");
            match cur {
                "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
                    if first_match == "" {
                        first_match = cur;
                    }
                    last_match = cur;
                },
                "one" | "two" | "three" | "four" | "five" | "six" | "seven" | "eight" | "nine" => {
                    let res = valuemap[cur];
                    if first_match == "" {
                        first_match = res;
                    }
                    last_match = res;
                },
                _ => continue
            }
        }
    };
    return first_match.to_owned() + last_match
}

fn main() {
    let content = fs::read_to_string("../question").expect("Should have been able to read file");
    let lines: Vec<&str> = content.split("\n").collect();
    let mut sum: u32 = 0;
    let mut sum2: u32 = 0;
    for line in lines {
        let ans = find_matches(line);
        sum += ans.parse::<u32>().unwrap();
        
        let ans2 = find_matches2(line);
        sum2 += ans2.parse::<u32>().unwrap();
        
    }

    println!("answer 1: {}", sum);
    println!("answer 2: {}", sum2);
}
