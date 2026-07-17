use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
advent_of_code::solution!(5);

pub fn extract_chars(input: &str) -> Vec<u8> {
    input
        .trim()
        .chars()
        .map(|c| u8::try_from(c).unwrap())
        .collect()
}

pub fn reduce(tuples: &[u8], ignore_char: Option<u8>) -> usize {
    let mut tuples = if let Some(ignore_char) = ignore_char {
        tuples
            .iter()
            .filter(|&&c| c != ignore_char && c != ignore_char + 32)
            .cloned()
            .collect()
    } else {
        tuples.to_vec()
    };
    loop {
        let indexes_to_remove: Vec<usize> = tuples
            .iter()
            .tuple_windows()
            .enumerate()
            // diff between a and A is 32
            .filter(|&(_, (a, b))| a.abs_diff(*b) == 32)
            .map(|(idx, _)| idx)
            .collect();

        if indexes_to_remove.is_empty() {
            return tuples.len();
        }
        let mut prev: Option<usize> = None;
        indexes_to_remove
            .iter()
            .filter(|&&x| {
                let drop = prev.is_some_and(|p| x == p.wrapping_add(1));
                prev = Some(x);
                !drop
            })
            .enumerate()
            .for_each(|(count, &i)| {
                let i = i - count * 2;
                tuples.drain(i..i + 2);
            });
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let tuples = extract_chars(input);
    Some(reduce(&tuples, None))
}

pub fn part_two(input: &str) -> Option<usize> {
    let tuples = extract_chars(input);
    (65..65 + 26)
        .into_par_iter()
        .map(|i| reduce(&tuples, Some(i)))
        .min()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
