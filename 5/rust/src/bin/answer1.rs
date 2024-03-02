use std::fs;

struct Question {
    seeds: Vec<u64>,
    maps: Vec<Vec<Vec<u64>>>,
}

fn main() {
    let question = Question::read("../question");

    let answer = question.solve();

    println!("answer is {answer}");
}

impl Question {
    fn read(path: &str) -> Question {
        let inp = fs::read_to_string(path).unwrap();

        let seeds = inp
            .lines()
            .take(1)
            .flat_map(|line| line.split_whitespace())
            .skip(1)
            .map(|val| val.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        println!("{seeds:?}");

        let maps = inp
            .split("\n\n")
            .skip(1)
            .map(|group| {
                group
                    .split('\n')
                    .skip(1)
                    .map(|line| {
                        line.split_whitespace()
                            .map(|val| val.parse::<u64>().unwrap())
                            .collect::<Vec<u64>>()
                    })
                    .filter(|line| !line.is_empty())
                    .collect::<Vec<Vec<u64>>>()
            })
            .collect::<Vec<Vec<Vec<u64>>>>();

        for group in &maps {
            println!("{group:?}")
        }

        Question { seeds, maps }
    }

    fn solve(&self) -> u64 {
        let mut smallest_soil: Option<u64> = None;

        for seed in &self.seeds {
            let soil = Self::convert(*seed, &self.maps);

            smallest_soil = match smallest_soil {
                Some(cur) => Some(std::cmp::min(soil, cur)),
                None => Some(soil),
            };
        }

        smallest_soil.unwrap()
    }

    fn convert(seed: u64, maps: &Vec<Vec<Vec<u64>>>) -> u64 {
        let mut cur_value = seed;
        for map in maps {
            cur_value = Self::apply_map(cur_value, map)
        }
        cur_value
    }

    fn apply_map(value: u64, map: &Vec<Vec<u64>>) -> u64 {
        let mut cur_value = value;

        for row in map {
            match Self::apply_row(cur_value, row) {
                Some(val) => {
                    cur_value = val;
                    break;
                }
                None => continue,
            };
        }

        cur_value
    }

    fn apply_row(value: u64, row: &Vec<u64>) -> Option<u64> {
        let into = row[0];
        let from = row[1];
        let range = row[2];

        if value >= from && value < (from + range) {
            Some(value - from + into)
        } else {
            None
        }
    }
}
