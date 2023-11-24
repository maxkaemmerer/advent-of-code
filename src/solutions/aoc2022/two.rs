use std::ops::Add;

use crate::common::file_to_lines;
use crate::solutions::aoc2022::two::Hand::{Paper, Rock, Scissors};
use crate::solutions::aoc2022::two::HandResult::{Draw, Loss, Win};

impl Add<HandResult> for Hand {
    type Output = u128;

    fn add(self, rhs: HandResult) -> Self::Output {
        self as u128 + rhs as u128
    }
}

enum HandResult {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

fn parse_hand(hand: &str) -> Option<Hand> {
    match hand {
        "A" | "X" => Some(Rock),
        "B" | "Y" => Some(Paper),
        "C" | "Z" => Some(Scissors),
        _ => None,
    }
}
fn parse_result(hand: &str) -> Option<HandResult> {
    match hand {
        "X" => Some(Loss),
        "Y" => Some(Draw),
        "Z" => Some(Win),
        _ => None,
    }
}

fn compare(hands: (&Hand, &Hand)) -> u128 {
    match hands {
        (Rock, Scissors) => Scissors + Loss,
        (Rock, Paper) => Paper + Win,
        (Rock, Rock) => Rock + Draw,
        (Paper, Scissors) => Scissors + Win,
        (Paper, Paper) => Paper + Draw,
        (Paper, Rock) => Rock + Loss,
        (Scissors, Scissors) => Scissors + Draw,
        (Scissors, Paper) => Paper + Loss,
        (Scissors, Rock) => Rock + Win,
    }
}

fn decide_hand(opponent_hand: &Hand, needed_result: &HandResult) -> Hand {
    match (opponent_hand, needed_result) {
        (Rock, Win) => Paper,
        (Rock, Draw) => Rock,
        (Rock, Loss) => Scissors,
        (Paper, Win) => Scissors,
        (Paper, Draw) => Paper,
        (Paper, Loss) => Rock,
        (Scissors, Win) => Rock,
        (Scissors, Draw) => Scissors,
        (Scissors, Loss) => Paper,
    }
}

pub fn solve_a(path: &str) -> u128 {
    let lines = file_to_lines(path);

    lines.iter().fold(0, |acc, line| {
        let mut hands = line.split_whitespace().into_iter();

        let tuple = (
            hands.next().and_then(parse_hand),
            hands.next().and_then(parse_hand),
        );

        if let (Some(opponent), Some(you)) = tuple {
            return acc + compare((&opponent, &you));
        }

        0
    })
}

pub fn solve_b(path: &str) -> u128 {
    let lines = file_to_lines(path);

    lines.iter().fold(0, |acc, line| {
        let mut chars = line.split_whitespace().into_iter();

        let tuple = (
            chars.next().and_then(parse_hand),
            chars.next().and_then(parse_result),
        );

        if let (Some(opponent_hand), Some(needed_result)) = tuple {
            return acc + compare((&opponent_hand, &decide_hand(&opponent_hand, &needed_result)));
        }

        0
    })
}
