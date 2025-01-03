use cached::proc_macro::cached;

advent_of_code::solution!(11);

#[cached]
pub fn find_power_level_of_one_fuel_cell(x: i32, y: i32, serial: i32) -> i32 {
    let rack_id = x + 10;
    let mut power_level = rack_id * y;
    power_level += serial;
    power_level *= rack_id;
    power_level /= 100;
    power_level %= 10;
    power_level -= 5;

    power_level
}

pub fn power_level_3x3(x: i32, y: i32, serial: i32) -> i32 {
    if x > 298 || y > 298 || x < 1 || y < 1 {
        return 0;
    }
    let offsets = [
        (0, 0),
        (1, 0),
        (2, 0),
        (0, 1),
        (1, 1),
        (2, 1),
        (0, 2),
        (1, 2),
        (2, 2),
    ];

    offsets
        .iter()
        .map(|(a, b)| (x + a, y + b))
        .map(|(x, y)| find_power_level_of_one_fuel_cell(x, y, serial))
        .sum()
}
pub fn power_level_n(x: i32, y: i32, serial: i32, n: u32) -> i32 {
    if x > (300 - n as i32 + 1) || y > (300 - n as i32 + 1) || x < 1 || y < 1 {
        return 0;
    }
    let mut offsets: Vec<(i32, i32)> = Vec::new();

    for y in 0..n as i32 {
        for x in 0..n as i32 {
            offsets.push((x, y));
        }
    }

    offsets
        .iter()
        .map(|(a, b)| (x + a, y + b))
        .map(|(x, y)| find_power_level_of_one_fuel_cell(x, y, serial))
        .sum()
}

fn calculate_partial_sums(grid: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let mut partial_sums = vec![vec![0; 300]; 300];
    (0..300).for_each(|y| {
        (0..300).for_each(|x| {
            partial_sums[y][x] = grid[y][x]
                + if x > 0 { partial_sums[y][x - 1] } else { 0 }
                + if y > 0 { partial_sums[y - 1][x] } else { 0 }
                - if x > 0 && y > 0 {
                    partial_sums[y - 1][x - 1]
                } else {
                    0
                };
        });
    });

    partial_sums
}

pub fn part_one(input: &str) -> Option<String> {
    let serial = input.trim().parse::<i32>().unwrap();
    let mut grid = vec![vec![0; 300]; 300];

    (0..300).for_each(|y| {
        (0..300).for_each(|x| {
            grid[y][x] = find_power_level_of_one_fuel_cell(x as i32, y as i32, serial);
        });
    });
    let partial_sums = calculate_partial_sums(&grid);
    let mut max_coord = (0, 0);
    let mut max = 0;
    let n = 3;
    (0..=(300 - n)).for_each(|y| {
        (0..=(300 - n)).for_each(|x| {
            let x2 = x + n - 1;
            let y2 = y + n - 1;
            let total = partial_sums[y2][x2]
                - if x > 0 { partial_sums[y2][x - 1] } else { 0 }
                - if y > 0 { partial_sums[y - 1][x2] } else { 0 }
                + if x > 0 && y > 0 {
                    partial_sums[y - 1][x - 1]
                } else {
                    0
                };

            if total > max {
                max = total;
                max_coord = (x, y);
            }
        });
    });

    Some(format!("{}, {}", max_coord.0, max_coord.1))
}

pub fn part_two(input: &str) -> Option<String> {
    let serial = input.trim().parse::<i32>().unwrap();
    let mut grid = vec![vec![0; 300]; 300];

    (0..300).for_each(|y| {
        (0..300).for_each(|x| {
            grid[y][x] = find_power_level_of_one_fuel_cell(x as i32, y as i32, serial);
        });
    });

    let partial_sums = calculate_partial_sums(&grid);

    let mut max = i32::MIN;
    let mut max_coord = (0, 0, 0);

    (1..=300).for_each(|n| {
        (0..=(300 - n)).for_each(|y| {
            (0..=(300 - n)).for_each(|x| {
                let x2 = x + n - 1;
                let y2 = y + n - 1;
                let total = partial_sums[y2][x2]
                    - if x > 0 { partial_sums[y2][x - 1] } else { 0 }
                    - if y > 0 { partial_sums[y - 1][x2] } else { 0 }
                    + if x > 0 && y > 0 {
                        partial_sums[y - 1][x - 1]
                    } else {
                        0
                    };

                if total > max {
                    max = total;
                    max_coord = (x, y, n);
                }
            });
        });
    });

    Some(format!("{},{},{}", max_coord.0, max_coord.1, max_coord.2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_fuel_cell_power_level() {
        assert_eq!(find_power_level_of_one_fuel_cell(3, 5, 8), 4);
    }

    #[test]
    fn test_part_one() {
        let result = part_one("42");
        assert_eq!(result, Some("21, 61".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two("18");
        assert_eq!(result, Some("90,269,16".to_string()));
    }
}
