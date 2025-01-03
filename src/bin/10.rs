use hashbrown::HashSet;
use nom::InputIter;

advent_of_code::solution!(10);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Drone {
    pub px: i32,
    pub py: i32,
    pub vx: i32,
    pub vy: i32,
}

fn pull_numbers_from_brackets(input: &str) -> (i32, i32) {
    let start = input.position(|c| c == '<').unwrap();
    let end = input.position(|c| c == '>').unwrap();
    let (a, b) = &input[start + 1..end].split_once(',').unwrap();
    (
        a.trim().parse::<i32>().unwrap(),
        b.trim().parse::<i32>().unwrap(),
    )
}

impl Drone {
    pub fn parse(input: &str) -> Self {
        // position=< 21373,  53216> velocity=<-2, -5>
        let (position, velocity) = input
            .split_once(" velocity=")
            .expect("Velocity not in input");
        let (px, py) = pull_numbers_from_brackets(position);
        let (vx, vy) = pull_numbers_from_brackets(velocity);

        Self { px, py, vx, vy }
    }

    pub fn move_once(&mut self) {
        self.px += self.vx;
        self.py += self.vy;
    }

    pub fn move_n_times(&mut self, n: u32) {
        self.px += n as i32 * self.vx;
        self.py += n as i32 * self.vy;
    }
}

fn print(drones: &[Drone]) -> String {
    let mut grid: HashSet<(i32, i32)> = HashSet::new();
    let x_min = drones.iter().map(|d| d.px).min().unwrap();
    let x_max = drones.iter().map(|d| d.px).max().unwrap() + 1;

    let y_min = drones.iter().map(|d| d.py).min().unwrap();
    let y_max = drones.iter().map(|d| d.py).max().unwrap() + 1;

    drones.iter().for_each(|d| {
        grid.insert((d.px, d.py));
    });

    let mut result = String::new();
    for y in y_min..y_max {
        let mut line = String::new();
        for x in x_min..x_max {
            let ch = if grid.contains(&(x, y)) { '#' } else { ' ' };
            line.push(ch);
        }
        result.push_str(line.as_str());
        result.push('\n');
    }

    result
}

fn drone_span(drones: &[Drone]) -> i32 {
    let x_min = drones.iter().map(|d| d.px).min().unwrap();
    let x_max = drones.iter().map(|d| d.px).max().unwrap();

    let y_min = drones.iter().map(|d| d.py).min().unwrap();
    let y_max = drones.iter().map(|d| d.py).max().unwrap();

    ((x_max - x_min) + (y_max - y_min)) / 2
}

pub fn part_one(input: &str) -> Option<String> {
    let mut drones: Vec<Drone> = input.lines().map(Drone::parse).collect();

    let start_offset = if drones.len() > 50 { 10500 } else { 0 }; // minimum before they would start aligning
    drones.iter_mut().for_each(|d| d.move_n_times(start_offset));
    let mut offset = 0;
    let mut previous_span = drone_span(&drones);

    let mut check_drones = drones.clone();
    loop {
        check_drones.iter_mut().for_each(|d| d.move_once());
        let new_span = drone_span(&check_drones);
        if new_span < previous_span {
            previous_span = new_span;
            offset += 1;
        } else {
            break;
        }
    }

    drones.iter_mut().for_each(|d| d.move_n_times(offset));
    // print(&drones);

    Some(print(&drones))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut drones: Vec<Drone> = input.lines().map(Drone::parse).collect();

    let start_offset = if drones.len() > 50 { 10500 } else { 0 }; // minimum before they would start aligning
    drones.iter_mut().for_each(|d| d.move_n_times(start_offset));
    let mut offset = 0;
    let mut previous_span = drone_span(&drones);

    let mut check_drones = drones.clone();
    loop {
        check_drones.iter_mut().for_each(|d| d.move_once());
        let new_span = drone_span(&check_drones);
        if new_span < previous_span {
            previous_span = new_span;
            offset += 1;
        } else {
            break;
        }
    }

    drones.iter_mut().for_each(|d| d.move_n_times(offset));
    // print(&drones);

    Some(offset + start_offset)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        let result = Drone::parse("position=< 21373,  53216> velocity=<-2, -5>");
        assert_eq!(
            result,
            Drone {
                px: 21373,
                py: 53216,
                vx: -2,
                vy: -5
            }
        )
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("#   #  ###\n#   #   # \n#   #   # \n#####   # \n#   #   # \n#   #   # \n#   #   # \n#   #  ###\n".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }
}
