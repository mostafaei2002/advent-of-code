use rayon::prelude::*;
use std::fs;

// 2.2 Billion Seeds
// 3.6 Billion Locations reverse

// Run without Paralellization , Close to 0 memory, 12.5% CPU, 4500sec => 1.15Hours
// Run with Rayon, Close to 0 memory, 100% CPU, 1700 Seconds => 26 mins
// Run with Rayon, Better Async, 1350 Sec => 23 Mins

struct Question {
    ranges: Vec<(u64, u64)>,
    maps: Vec<Vec<Vec<u64>>>,
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    {
        let question = Question::read("../question");

        let answer = question.solve();

        println!("answer is {answer}");
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?} seconds", elapsed.as_secs());
}

impl Question {
    fn read(path: &str) -> Question {
        let inp = fs::read_to_string(path).unwrap();

        let seeds = inp
            .lines()
            .take(1)
            .flat_map(|line| line.split_whitespace())
            .skip(1)
            .map(|val| val.parse::<u64>().unwrap());

        let ranges = seeds
            .clone()
            .step_by(2)
            .zip(seeds.skip(1).step_by(2))
            .collect();

        println!("{ranges:?}");

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

        Question { ranges, maps }
    }

    fn solve(&self) -> u64 {
        let smallest_soil = self
            .ranges
            .par_iter() // Parallel iterator over ranges
            .map(|range| {
                let start = range.0;
                let end = start + range.1;

                println!("going through {}-{} a range of {}", start, end, range.1);

                (start..end)
                    .into_par_iter()
                    .map(|seed| Self::convert(seed, &self.maps))
                    .min() // Find minimum soil value within the range
            })
            .min(); // Find minimum soil value across all ranges

        smallest_soil.unwrap().unwrap()
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
