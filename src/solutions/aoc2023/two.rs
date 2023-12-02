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
    red: 12,
};

struct Game<'a> {
    id: usize,
    sets: Vec<Set<'a>>,
}

type Set<'a> = Vec<Token<'a, usize>>;

fn parse_game(line: &str) -> Game {
    let sets = line
        .split(';')
        .map(|set| {
            let mut tokens = vec![];

            if let Some(green) = parse_token_before(set, "green", " ", ' ') {
                tokens.push(green);
            }
            if let Some(red) = parse_token_before(set, "red", " ", ' ') {
                tokens.push(red);
            }
            if let Some(blue) = parse_token_before(set, "blue", " ", ' ') {
                tokens.push(blue);
            }

            tokens
        })
        .collect();

    let id: Token<usize> = parse_token(line, "Game", " ", ':').expect("Missing game id");

    Game { id: id.1, sets }
}

fn is_valid_set_for_color(set: &Set, token_type: &str, limit: usize) -> Option<bool> {
    set.iter()
        .find(|a| a.to_owned().0 == token_type)
        .map(|count| count.1 <= limit)
}

fn is_valid_set(set: &Set, limits: &Limits) -> bool {
    let green = is_valid_set_for_color(set, "green", limits.green);
    let red = is_valid_set_for_color(set, "red", limits.red);
    let blue = is_valid_set_for_color(set, "blue", limits.blue);

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

    lines
        .iter()
        .map(|line| parse_game(line))
        .filter(|game| is_valid_game(game, A_LIMITS))
        .map(|game| game.id)
        .sum()
}

fn is_valid_game(game: &Game, limits: Limits) -> bool {
    game.sets
        .iter()
        .filter(|set| is_valid_set(set, &limits))
        .count()
        == game.sets.len()
}

fn min_token_count_of_set(game: &Game, token_type: &str) -> usize {
    game.sets
        .iter()
        .map(|set| {
            set.iter()
                .filter(|token| token.0 == token_type)
                .map(|token| token.1)
                .max()
                .unwrap_or_default()
        })
        .max()
        .unwrap_or_default()
}

fn power_of_game(game: Game) -> usize {
    let min_red: usize = min_token_count_of_set(&game, "red");
    let min_green: usize = min_token_count_of_set(&game, "green");
    let min_blue: usize = min_token_count_of_set(&game, "blue");

    min_red * min_green * min_blue
}

pub fn solve_b(path: &str) -> usize {
    let lines = file_to_lines(path);

    lines
        .iter()
        .map(|line| power_of_game(parse_game(line)))
        .sum()
}
