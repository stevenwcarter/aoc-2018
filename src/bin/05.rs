use itertools::Itertools;
advent_of_code::solution!(5);

// 32 diff between a and A
pub fn part_one(input: &str) -> Option<usize> {
    let mut tuples: Vec<u8> = input
        .trim()
        .chars()
        .map(|c| u8::try_from(c).unwrap())
        .collect();
    loop {
        let mut indexes_to_remove: Vec<usize> = tuples
            .iter()
            .tuple_windows()
            .enumerate()
            .filter(|&(_, (a, b))| a.abs_diff(*b) == 32)
            .map(|(idx, _)| idx)
            .collect();

        if indexes_to_remove.is_empty() {
            return Some(tuples.len());
        }
        let mut prev: Option<usize> = None;
        indexes_to_remove.retain(|&x| {
            let drop = prev.is_some_and(|p| x == p.wrapping_add(1));

            prev = Some(x);

            !drop
        });
        indexes_to_remove
            .iter()
            .enumerate()
            .for_each(|(count, &i)| {
                let i = i - count * 2;
                tuples.drain(i..i + 2);
            });
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    None
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
        assert_eq!(result, None);
    }
}
