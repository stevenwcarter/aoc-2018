advent_of_code::solution!(8);

fn parse(input: &str) -> impl Iterator<Item = u32> + '_ {
    input.split_ascii_whitespace().map(|n| n.parse().unwrap())
}

fn metadata_sum(it: &mut impl Iterator<Item = u32>) -> u32 {
    let children = it.next().unwrap();
    let metas = it.next().unwrap();
    let child_sum: u32 = (0..children).map(|_| metadata_sum(it)).sum();
    let meta_sum: u32 = (0..metas).map(|_| it.next().unwrap()).sum();
    child_sum + meta_sum
}

fn node_value(it: &mut impl Iterator<Item = u32>) -> u32 {
    let children = it.next().unwrap() as usize;
    let metas = it.next().unwrap();
    let child_values: Vec<u32> = (0..children).map(|_| node_value(it)).collect();
    (0..metas)
        .map(|_| it.next().unwrap() as usize)
        .map(|m| {
            if children == 0 {
                m as u32
            } else {
                child_values.get(m.wrapping_sub(1)).copied().unwrap_or(0)
            }
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(metadata_sum(&mut parse(input)))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(node_value(&mut parse(input)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(138));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(66));
    }
}
