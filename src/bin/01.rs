use hashbrown::HashSet;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i32> {
    Some(input.lines().map(|l| l.parse::<i32>().unwrap()).sum())
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut seen: HashSet<i32> = HashSet::new();
    let numbers: Vec<i32> = input.lines().map(|l| l.parse::<i32>().unwrap()).collect();

    let mut frequency = 0;

    for index in 0.. {
        let index = index % numbers.len();
        frequency += numbers[index];

        if !seen.insert(frequency) {
            break;
        }
    }
    Some(frequency)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
