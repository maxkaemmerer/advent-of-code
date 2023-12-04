use crate::common;
use crate::common::file_to_lines;
use crate::tokens::{parse_token_value_after};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Card {
    id: usize,
    cards_to_process: Vec<usize>,
}

impl Card {
    pub fn score(&self) -> usize {
        self.cards_to_process
            .iter()
            .fold(0, |acc, _| if acc == 0 { 1 } else { acc * 2 })
    }

    pub fn won_cards<'a>(&self, all_cards: &'a HashMap<usize, Card>) -> Vec<&'a Card> {
        self.cards_to_process
            .iter()
            .map(|card_to_process| {
                all_cards
                    .get(card_to_process)
                    .expect("error in the algorithm, we are looking for a card that does not exist")
            })
            .collect()
    }
}

fn parse_scores(string: &str) -> HashSet<usize> {
    HashSet::from_iter(string.split_whitespace().map(|score| {
        score
            .parse::<usize>()
            .expect("invalid input, non number score given")
    }))
}

fn calculate_cards(lines: Vec<&str>) -> HashMap<usize, Card> {
    lines
        .iter()
        .map(|line| {
            let card_index = line.find(':').expect("Invalid input, must include :") + 1;
            let card_id: usize = parse_token_value_after(
                common::remove_multiple_whitespaces(line).as_str(),
                "Card",
                " ",
                ":",
            )
            .expect("Invalid input must include card id")
            .1;
            let scores = &line[card_index..];

            let split_scores: Vec<&str> = scores.split("|").collect();
            if let [winning_scores, your_scores] = split_scores[..] {
                let winning_scores: HashSet<usize> = parse_scores(winning_scores);
                let player_scores: HashSet<usize> = parse_scores(your_scores);

                let intersections = winning_scores.intersection(&player_scores);
                (
                    card_id,
                    Card {
                        id: card_id,
                        cards_to_process: (card_id + 1..card_id + 1 + intersections.count())
                            .collect(),
                    },
                )
            } else {
                panic!("Invalid input, must include score separator |");
            }
        })
        .collect()
}

pub fn solve_a(path: &str) -> usize {
    let lines = file_to_lines(path);
    let cards = calculate_cards(lines.iter().map(|line| line.as_str()).collect());

    cards
        .iter()
        .fold(0, |total, (_, card)| total + card.score())
}

pub fn solve_b(path: &str) -> usize {
    let lines = file_to_lines(path);
    let initial_cards = calculate_cards(lines.iter().map(|line| line.as_str()).collect());
    let mut card_results: HashMap<usize, usize> = HashMap::new();

    let total_won_cards: usize = initial_cards
        .iter()
        .map(|(_, card)| total_won_cards_for_card(card, &initial_cards, &mut card_results))
        .sum();

    total_won_cards + initial_cards.len()
}

fn total_won_cards_for_card(
    card: &Card,
    all_cards: &HashMap<usize, Card>,
    card_results: &mut HashMap<usize, usize>,
) -> usize {
    if let Some(result) = card_results.get(&card.id) {
        return result.clone();
    }

    let won_cards = card.won_cards(&all_cards);

    let total_cards_for_card: usize = won_cards
        .iter()
        .map(|won_card| total_won_cards_for_card(won_card, all_cards, card_results))
        .sum();

    let total_cards_processed_for_card = total_cards_for_card + won_cards.len();

    card_results.insert(card.id, total_cards_processed_for_card);

    total_cards_processed_for_card
}
