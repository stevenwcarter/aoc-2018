#![allow(unused_assignments)]
advent_of_code::solution!(7);
use std::collections::BTreeSet;

use hashbrown::{HashMap, HashSet};

pub fn extract_prerequisites(input: &str) -> HashMap<char, HashSet<char>> {
    let char_map: Vec<(char, char)> = input
        .lines()
        .map(|s| {
            let s: Vec<char> = s.chars().collect();
            (*s.get(5).unwrap(), *s.get(36).unwrap())
        })
        .collect();
    let mut prerequisites: HashMap<char, HashSet<char>> = HashMap::new();
    char_map.iter().for_each(|(prereq, item)| {
        prerequisites.entry(*prereq).or_insert(HashSet::new());
        prerequisites
            .entry(*item)
            .or_insert(HashSet::new())
            .insert(*prereq);
    });

    prerequisites
}
pub fn part_one(input: &str) -> Option<String> {
    let mut prerequisites = extract_prerequisites(input);
    // let starting_length = prerequisites.len();
    let mut order = String::new();

    while prerequisites.len() > 0 {
        let mut next: Option<char> = None;
        let mut possibles: Vec<&char> = prerequisites
            .iter()
            .filter(|(_, v)| v.is_empty())
            .map(|(k, _)| k)
            .collect();
        possibles.sort();
        next = Some(**possibles.first().unwrap());
        order.push(next.unwrap());
        prerequisites.remove(&next.unwrap());
        prerequisites.iter_mut().for_each(|(_, v)| {
            v.remove(&next.unwrap());
        })
    }

    Some(order)
}

#[derive(Debug, Clone)]
pub struct Worker {
    letter: char,
    seconds_remaining: u64,
}

#[derive(Debug, Clone)]
pub struct WorkerPool {
    time: u64,
    worker_count: u8,
    pool: Vec<Worker>,
    available: BTreeSet<char>,
    prerequisites: HashMap<char, HashSet<char>>,
}

impl WorkerPool {
    pub fn new(prerequisites: HashMap<char, HashSet<char>>, worker_count: u8) -> Self {
        Self {
            time: 0,
            worker_count,
            pool: Vec::new(),
            available: BTreeSet::new(),
            prerequisites,
        }
    }

    pub fn tick(&mut self) {
        let mut time_advancement = 1;
        if self.pool.len() == self.worker_count as usize {
            time_advancement = self.pool.iter().map(|w| w.seconds_remaining).min().unwrap();
        }
        self.time += time_advancement;
        let mut to_remove: Option<Vec<char>> = None;
        self.pool.iter_mut().for_each(|w| {
            w.seconds_remaining -= time_advancement;
            if w.seconds_remaining == 0 {
                if to_remove.is_none() {
                    to_remove = Some(vec![w.letter]);
                } else {
                    to_remove.as_mut().unwrap().push(w.letter);
                }
            }
        });
        if let Some(to_remove) = to_remove {
            to_remove.iter().for_each(|chosen| {
                self.pool.retain(|w| w.letter != *chosen);
                self.prerequisites.remove(chosen);
                self.prerequisites.iter_mut().for_each(|(_, v)| {
                    v.remove(chosen);
                });
            });
            let in_progress: Vec<char> = self.pool.iter().map(|w| w.letter).collect();
            self.available = self
                .prerequisites
                .iter()
                .filter(|(_, v)| v.is_empty())
                .map(|(k, _)| k)
                .filter(|k| !in_progress.contains(k))
                .cloned()
                .collect();
        }
    }

    fn process(&mut self) {
        self.available = self
            .prerequisites
            .iter()
            .filter(|(_, v)| v.is_empty())
            .map(|(k, _)| k)
            .cloned()
            .collect();
        while !self.prerequisites.is_empty() || !self.available.is_empty() {
            if !self.available.is_empty() {
                let available_workers = self.worker_count as usize - self.pool.len();
                if available_workers > 0 {
                    let mut possibles: Vec<&char> = self
                        .prerequisites
                        .iter()
                        .filter(|(_, v)| v.is_empty())
                        .map(|(k, _)| k)
                        .collect();
                    possibles.sort();

                    let count_to_process = available_workers.min(self.available.len());
                    (0..count_to_process).for_each(|_| {
                        let letter = self.available.pop_first().unwrap();

                        let base = if self.worker_count == 2 { 0 } else { 60 };

                        self.pool.push(Worker {
                            letter: letter,
                            seconds_remaining: (letter as u8 as u64) - 64 + base,
                        });
                    })
                }
            }
            self.tick();
        }
    }
}
pub fn part_two(input: &str) -> Option<u64> {
    let worker_count = if input.len() < 900 { 2 } else { 5 };
    let prerequisites = extract_prerequisites(input);

    let mut workers = WorkerPool::new(prerequisites, worker_count);

    workers.process();

    Some(workers.time)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(String::from("CABDFE")));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(15));
    }
}
