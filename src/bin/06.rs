advent_of_code::solution!(6);

#[derive(Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn manhattan(self, x: i32, y: i32) -> i32 {
        (self.x - x).abs() + (self.y - y).abs()
    }
}

pub fn parse(input: &str) -> Vec<Point> {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| {
            let (x, y) = line.split_once(',').expect("coordinate needs a comma");
            Point {
                x: x.trim().parse().expect("x should be an integer"),
                y: y.trim().parse().expect("y should be an integer"),
            }
        })
        .collect()
}

fn bounds(points: &[Point]) -> (i32, i32, i32, i32) {
    let min_x = points.iter().map(|p| p.x).min().unwrap();
    let max_x = points.iter().map(|p| p.x).max().unwrap();
    let min_y = points.iter().map(|p| p.y).min().unwrap();
    let max_y = points.iter().map(|p| p.y).max().unwrap();
    (min_x, min_y, max_x, max_y)
}

fn nearest(points: &[Point], x: i32, y: i32) -> Option<usize> {
    let mut best = i32::MAX;
    let mut owner = None;
    let mut tied = false;

    for (i, p) in points.iter().enumerate() {
        let d = p.manhattan(x, y);
        if d < best {
            best = d;
            owner = Some(i);
            tied = false;
        } else if d == best {
            tied = true;
        }
    }

    if tied {
        None
    } else {
        owner
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let points = parse(input);
    let (min_x, min_y, max_x, max_y) = bounds(&points);

    let mut area = vec![0u64; points.len()];
    let mut infinite = vec![false; points.len()];

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let Some(owner) = nearest(&points, x, y) else {
                continue; // tie — this cell is owned by no point
            };
            area[owner] += 1;
            if x == min_x || x == max_x || y == min_y || y == max_y {
                infinite[owner] = true;
            }
        }
    }

    area.iter()
        .zip(&infinite)
        .filter(|&(_, &is_infinite)| !is_infinite)
        .map(|(&a, _)| a)
        .max()
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(region_size(&parse(input), 10_000))
}

pub fn region_size(points: &[Point], threshold: i32) -> u64 {
    let (min_x, min_y, max_x, max_y) = bounds(points);

    let pad = threshold / points.len() as i32;

    let mut count = 0;
    for y in (min_y - pad)..=(max_y + pad) {
        for x in (min_x - pad)..=(max_x + pad) {
            let total: i32 = points.iter().map(|p| p.manhattan(x, y)).sum();
            if total < threshold {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(17));
    }

    #[test]
    fn test_part_two() {
        let points = parse(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(region_size(&points, 32), 16);
    }
}
