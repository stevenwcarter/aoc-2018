use hashbrown::{HashMap, HashSet};

advent_of_code::solution!(12);

fn find_mask(vals: &[bool]) -> u8 {
    let mut val = 0;
    vals.iter().for_each(|&v| {
        if v {
            val += 1;
        }
        val <<= 1;
    });
    val >>= 1;

    val
}

fn next_generation(plants: &HashMap<i64, bool>, rules: &HashSet<u8>) -> HashMap<i64, bool> {
    let mut result = HashMap::new();

    let start = plants.keys().min().unwrap() - 2;
    let end = plants.keys().max().unwrap() + 4;

    (start..end).for_each(|pos| {
        let checks = vec![
            *plants.get(&(pos - 2)).unwrap_or(&false),
            *plants.get(&(pos - 1)).unwrap_or(&false),
            *plants.get(&(pos)).unwrap_or(&false),
            *plants.get(&(pos + 1)).unwrap_or(&false),
            *plants.get(&(pos + 2)).unwrap_or(&false),
        ];
        let mask = find_mask(&checks);
        result.insert(pos, rules.contains(&mask));
    });

    result
}

fn print_plants(step: u8, plants: &HashMap<i64, bool>) {
    let start = -3;
    let end = 36;

    let mut result: String = String::new();
    result.push_str(&format!("{:2>}: ", step));

    (start..end).for_each(|pos| {
        let ch = match plants.get(&pos) {
            Some(true) => '#',
            _ => '.',
        };
        result.push(ch);
    });

    println!("{result}");
}

pub fn part_one(input: &str) -> Option<i64> {
    let input: Vec<&str> = input.lines().collect();
    let mut rules: HashSet<u8> = HashSet::new();
    input[2..].iter().for_each(|l| {
        let (rule, result) = l.split_once(" => ").unwrap();
        if result == "#" {
            let rule = rule.replace(".", "0").replace("#", "1");
            let rule = u8::from_str_radix(&rule, 2).expect("Could not parse {rule}");
            rules.insert(rule);
        }
    });
    let mut plants: HashMap<i64, bool> = HashMap::new();
    input[0][15..].chars().enumerate().for_each(|(index, ch)| {
        match ch {
            '#' => plants.insert(index as i64, true),
            '.' => plants.insert(index as i64, false),
            _ => unreachable!("Invalid input {ch}"),
        };
    });
    // print_plants(0, &plants);
    (0..20).for_each(|s| {
        plants = next_generation(&plants, &rules);
        // print_plants(s + 1, &plants);
    });

    Some(
        plants
            .iter()
            .filter(|(_, v)| **v)
            .map(|(&k, _)| k as i64)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    let input: Vec<&str> = input.lines().collect();
    let mut rules: HashSet<u8> = HashSet::new();
    input[2..].iter().for_each(|l| {
        let (rule, result) = l.split_once(" => ").unwrap();
        if result == "#" {
            let rule = rule.replace(".", "0").replace("#", "1");
            let rule = u8::from_str_radix(&rule, 2).expect("Could not parse {rule}");
            rules.insert(rule);
        }
    });
    let mut plants: HashMap<i64, bool> = HashMap::new();
    input[0][15..].chars().enumerate().for_each(|(index, ch)| {
        match ch {
            '#' => plants.insert(index as i64, true),
            '.' => plants.insert(index as i64, false),
            _ => unreachable!("Invalid input {ch}"),
        };
    });
    let mut prev_total: i64 = plants.iter().filter(|(_, v)| **v).map(|(&k, _)| k).sum();
    let mut diff = 0;

    // find sequence it increases by after stabilizing
    (0..3).for_each(|_| {
        (0..100).for_each(|_| {
            plants = next_generation(&plants, &rules);
        });
        let total: i64 = plants.iter().filter(|(_, v)| **v).map(|(&k, _)| k).sum();
        // println!("{} {total} {}", l * 100, total - prev_total);
        diff = total - prev_total;
        prev_total = total;
    });

    // compute the remainder based on the sequence
    Some((50_000_000_000 - 300) / 100 * diff + prev_total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_mask() {
        let masks = vec![true, true, true, true, true];
        assert_eq!(find_mask(&masks), 31);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(325));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
