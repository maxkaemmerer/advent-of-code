use crate::common::file_to_lines;
use crate::tokens::{parse_token, parse_token_before, Token};

struct Limits {
    green: usize,
    red: usize,
    blue: usize,
}

const A_LIMITS: Limits = Limits {
    green: 13,
    blue: 14,
    red: 12
};

type Game<'a> = (usize, Vec<Set<'a>>);


type Set<'a> = Vec<Token<'a>>;

fn parse_game<'a>(line: &'a str) -> Game {
    let tokens = line.split(';').map(|set| {
        let mut tokens = vec![];


        if let Some(green) = parse_token_before(set,"green"," ", ' ') {
            tokens.push(green);
        }
        if let Some(red) = parse_token_before(set,"red"," ", ' ') {
            tokens.push(red);
        }
        if let Some(blue) = parse_token_before(set,"blue"," ", ' ') {
            tokens.push(blue);
        }

        tokens
    }).collect();

    let id = parse_token(line, "Game", " ", ':').and_then(|token| token.1.parse::<usize>().ok()).expect("Missing game id");

    (id, tokens)
}

fn is_valid_set_for_color(set: &Set, token_type: &str, limit: usize) -> Option<bool> {
    set.iter().find(|a| a.to_owned().0 == token_type).and_then(|token| token.1.parse::<usize>().ok()).map(|count| count <= limit)
}

fn is_valid_set(set: &Set, limits: &Limits) -> bool {
    let green =  is_valid_set_for_color(set, "green", limits.green);
    let red =  is_valid_set_for_color(set, "red", limits.red);
    let blue =  is_valid_set_for_color(set, "blue", limits.blue);

    if let Some(false) = green {
        return false;
    }

    if let Some(false) = red {
        return false;
    }

    if let Some(false) = blue {
        return false;
    }

    true
}

pub fn solve_a(path: &str) -> usize {
    let lines = file_to_lines(path);

    lines.iter().map(|line| parse_game(line)).filter(|game| {
        let is_valid = is_valid_game(game, A_LIMITS);
        println!("{:?} {is_valid}", game);
        is_valid
    }).map(|game| game.0).sum()
}

fn is_valid_game(game: &Game, limits: Limits) -> bool {
    game.1.iter().filter(|set| is_valid_set(set, &limits)).count() == game.1.len()
}

pub fn solve_b(_: &str) -> usize {
    0
}