use std::collections::VecDeque;

use hashbrown::HashMap;

use itertools::Itertools;

advent_of_code::solution!(9);

fn parse_input(input: &str) -> (usize, u32) {
    let parts: Vec<&str> = input.split(' ').collect();
    let players = parts[0].parse::<usize>().expect("Could not parse players");
    let marbles = parts[6].parse::<u32>().expect("could not parse marbles");
    (players, marbles)
}

pub struct Game {
    pub player_count: usize,
    pub current_index: usize,
    pub next_marble: u32,
    pub marbles: VecDeque<u32>,
    pub last_marble: u32,
    pub scores: HashMap<u32, u32>,
}

impl Game {
    pub fn new_game(player_count: usize, last_marble: u32) -> Self {
        // TODO: figure out actual capacity, 1/23rd of this most likely
        let mut marbles = VecDeque::with_capacity(last_marble as usize);
        marbles.push_front(0);
        Self {
            player_count,
            current_index: 0,
            next_marble: 1,
            marbles,
            last_marble,
            scores: HashMap::new(),
        }
    }
    pub fn add_marble(&mut self) -> bool {
        if self.next_marble % 23 != 0 {
            self.marbles.rotate_left(1);
            self.marbles.push_back(self.next_marble);
        } else {
            self.marbles.rotate_right(7);
            let removed_marble = self.marbles.pop_back().unwrap();
            self.marbles.rotate_left(1);
            *self
                .scores
                .entry(self.next_marble % self.player_count as u32)
                .or_insert(0) += self.next_marble + removed_marble;
        }

        self.next_marble += 1;
        self.next_marble <= self.last_marble
    }

    pub fn winning_score(&self) -> u32 {
        *self.scores.values().max().unwrap()
    }

    pub fn print(&self) {
        let output = self
            .marbles
            .iter()
            .enumerate()
            .map(|(index, m)| {
                if index == self.current_index {
                    format!("({m})")
                } else {
                    format!("{m}")
                }
            })
            .join(" ");

        println!("{output}");
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (player_count, last_marble) = parse_input(input);

    println!("{player_count} {last_marble}");

    let mut game = Game::new_game(player_count, last_marble);

    while game.add_marble() {
        // game.print()
    }

    // > 365934
    Some(game.winning_score())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (player_count, last_marble) = parse_input(input);
    let last_marble = last_marble * 100;

    println!("{player_count} {last_marble}");

    let mut game = Game::new_game(player_count, last_marble);

    while game.add_marble() {
        // game.print()
    }

    // > 365934
    Some(game.winning_score())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1618() {
        let mut game = Game::new_game(10, 1618);
        while game.add_marble() {}

        assert_eq!(game.winning_score(), 8317);
    }
    #[test]
    fn test_7999() {
        let mut game = Game::new_game(13, 7999);
        while game.add_marble() {}

        assert_eq!(game.winning_score(), 146373);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(32));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22563));
    }
}
