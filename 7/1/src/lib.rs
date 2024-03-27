use std::{cmp::Ordering, collections::HashMap, fs};

pub struct Question {
    rows: Vec<Row>,
}

impl Question {
    pub fn build(path: &str) -> Self {
        let mut question: Vec<Row> = fs::read_to_string(path)
            .unwrap()
            .lines()
            .map(|line| line.split_once(' ').unwrap())
            .map(|row| Row {
                hand: Hand::from(row.0),
                bid: row.1.parse::<usize>().unwrap(),
            })
            .collect();

        question.sort_by(|a, b| a.hand.cmp(&b.hand));

        Self { rows: question }
    }

    pub fn solve(&self) -> usize {
        let mut answer: usize = 0;

        for (i, v) in self.rows.iter().enumerate() {
            answer += v.bid * (i + 1);
        }

        answer
    }
}

pub struct Row {
    hand: Hand,
    bid: usize,
}

#[derive(Debug, PartialEq, PartialOrd, Eq)]
pub enum Hands {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(PartialEq, Eq, PartialOrd, Debug)]
pub struct Hand(pub Vec<Card>);

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.get_type() > other.get_type() {
            return Ordering::Greater;
        } else if other.get_type() > self.get_type() {
            return Ordering::Less;
        }

        for i in 0..5 {
            if self.0[i].cmp(&other.0[i]) != Ordering::Equal {
                return self.0[i].cmp(&other.0[i]);
            }
        }

        Ordering::Equal
    }
}

impl Hand {
    pub fn get_type(&self) -> Hands {
        let mut counts: Vec<usize> = self.get_counts().into_iter().map(|v| v.1).collect();
        counts.sort();
        counts.reverse();

        match counts.as_slice() {
            &[a, ..] if a == 5 => Hands::FiveOfAKind,
            &[a, ..] if a == 4 => Hands::FourOfAKind,
            &[a, b, ..] if a == 3 && b == 2 => Hands::FullHouse,
            &[a, ..] if a == 3 => Hands::ThreeOfAKind,
            &[a, b, ..] if a == 2 && b == 2 => Hands::TwoPair,
            &[a, ..] if a == 2 => Hands::OnePair,
            _ => Hands::HighCard,
        }
    }

    pub fn get_counts(&self) -> HashMap<&Card, usize> {
        let mut counts: HashMap<&Card, usize> = HashMap::new();

        for card in &self.0 {
            *counts.entry(card).or_insert(0) += 1;
        }

        counts
    }
}

impl From<&str> for Hand {
    fn from(s: &str) -> Self {
        let mut new_hand_cards = Vec::new();
        for c in s.chars() {
            new_hand_cards.push(Card(c));
        }

        Self(new_hand_cards)
    }
}

#[derive(PartialOrd, PartialEq, Eq, Debug, Hash)]
pub struct Card(pub char);

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        let orders = String::from("23456789TJQKA");

        if orders.find(self.0).unwrap() > orders.find(other.0).unwrap() {
            Ordering::Greater
        } else if orders.find(self.0).unwrap() < orders.find(other.0).unwrap() {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}
