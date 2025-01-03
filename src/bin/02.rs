use advent_of_code::check_for_repeats_vec;

use itertools::Itertools;

advent_of_code::solution!(2);

fn compare_letters(combo: &[Vec<u8>]) -> Option<String> {
    let mut diffs = 0;

    for index in 0..combo[0].len() {
        if combo[0][index] != combo[1][index] {
            diffs += 1;
        }
    }

    if diffs == 1 {
        let mut result = String::new();
        for index in 0..combo[0].len() {
            if combo[0][index] == combo[1][index] {
                result.push(combo[0][index] as char)
            }
        }
        Some(result)
    } else {
        None
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut counts_2 = 0;
    let mut counts_3 = 0;

    input
        .lines()
        .map(check_for_repeats_vec)
        .for_each(|(twos, threes)| {
            if twos {
                counts_2 += 1
            }
            if threes {
                counts_3 += 1
            }
        });

    Some(counts_2 * counts_3)
}

pub fn part_two(input: &str) -> Option<String> {
    input
        .lines()
        .map(|s| s.chars().map(|c| c as u8).collect::<Vec<u8>>())
        .combinations(2)
        .filter_map(|combo| compare_letters(&combo))
        .next()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("abcde".to_string()));
    }
}
