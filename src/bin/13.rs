// TODO - come back and create a video of the track
use anyhow::{bail, Result};
use aoc_mine::Coord;
use hashbrown::HashMap;
use std::str::FromStr;

advent_of_code::solution!(13);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MapType {
    Intersection,
    Vertical,
    Horizontal,
    LeftToBottom,
    LeftToTop,
    Space,
}

use MapType::*;

impl MapType {
    fn from_char(c: char) -> Result<Self> {
        match c {
            '|' | '^' | 'v' => Ok(Vertical),
            '-' | '<' | '>' => Ok(Horizontal),
            '+' => Ok(Intersection),
            '/' => Ok(LeftToTop),
            '\\' => Ok(LeftToBottom),
            ' ' => Ok(Space),
            _ => bail!("unrecognized input"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CartTravelDirection {
    Up,
    Down,
    Left,
    Right,
}

impl CartTravelDirection {
    pub fn turn_left(&mut self) {
        *self = match *self {
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
            Self::Right => Self::Up,
        }
    }
    pub fn turn_right(&mut self) {
        *self = match *self {
            Self::Up => Self::Right,
            Self::Left => Self::Up,
            Self::Down => Self::Left,
            Self::Right => Self::Down,
        }
    }
    pub fn as_char(&self) -> char {
        match self {
            CartTravelDirection::Up => '^',
            CartTravelDirection::Down => 'v',
            CartTravelDirection::Left => '<',
            CartTravelDirection::Right => '>',
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CartDirectionChoice {
    Left,
    Straight,
    Right,
}

use CartDirectionChoice::*;

impl CartDirectionChoice {
    pub fn next(&mut self) -> Self {
        let current = *self;
        *self = match current {
            Left => Straight,
            Straight => Right,
            Right => Left,
        };

        current
    }
}

#[derive(Debug, Clone)]
pub struct Cart {
    pub position: Coord<usize>,
    pub direction: CartTravelDirection,
    pub next_direction: CartDirectionChoice,
}

impl Cart {
    pub fn new(x: usize, y: usize, direction: CartTravelDirection) -> Self {
        Self {
            position: Coord::new(x, y),
            direction,
            next_direction: CartDirectionChoice::Left,
        }
    }
    pub fn next_cart_position(&mut self, map: &HashMap<Coord<usize>, MapType>) {
        let position = &mut self.position;
        match self.direction {
            CartTravelDirection::Up => position.move_up(),
            CartTravelDirection::Down => position.move_down(),
            CartTravelDirection::Left => position.move_left(),
            CartTravelDirection::Right => position.move_right(),
        }

        let map_type = map.get(position).unwrap();
        match map_type {
            LeftToBottom => match self.direction {
                CartTravelDirection::Right => {
                    self.direction = CartTravelDirection::Down;
                }
                CartTravelDirection::Left => {
                    self.direction = CartTravelDirection::Up;
                }
                CartTravelDirection::Up => {
                    self.direction = CartTravelDirection::Left;
                }
                CartTravelDirection::Down => {
                    self.direction = CartTravelDirection::Right;
                }
            },
            LeftToTop => match self.direction {
                CartTravelDirection::Left => {
                    self.direction = CartTravelDirection::Down;
                }
                CartTravelDirection::Right => {
                    self.direction = CartTravelDirection::Up;
                }
                CartTravelDirection::Down => {
                    self.direction = CartTravelDirection::Left;
                }
                CartTravelDirection::Up => {
                    self.direction = CartTravelDirection::Right;
                }
            },
            Vertical | Horizontal => {
                // no-op, continue as facing
            }
            Intersection => {
                match self.next_direction.next() {
                    Left => self.direction.turn_left(),
                    Straight => {
                        // do nothing, continue in same direction},
                    }
                    Right => self.direction.turn_right(),
                };
            }
            Space => unreachable!("this is not supposed to be a space"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GridState {
    pub carts: Vec<Cart>,
    pub map: HashMap<Coord<usize>, MapType>,
}

impl FromStr for GridState {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut carts = Vec::new();
        let mut map = HashMap::new();

        s.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, char)| {
                match char {
                    '^' => carts.push(Cart::new(x, y, CartTravelDirection::Up)),
                    'v' => carts.push(Cart::new(x, y, CartTravelDirection::Down)),
                    '<' => carts.push(Cart::new(x, y, CartTravelDirection::Left)),
                    '>' => carts.push(Cart::new(x, y, CartTravelDirection::Right)),
                    _ => {}
                }
                map.insert(Coord::new(x, y), MapType::from_char(char).unwrap());
            })
        });

        Ok(Self { carts, map })
    }
}

impl GridState {
    pub fn simulate_part1(&mut self) -> Option<String> {
        loop {
            // Carts tick in reading order (top-to-bottom, left-to-right).
            self.carts
                .sort_unstable_by_key(|cart| (cart.position.y(), cart.position.x()));

            // Move one cart at a time; a collision is detected the instant a
            // cart steps onto another cart's current cell. Checking only after
            // every cart has moved would miss head-on swaps (two carts trading
            // cells in the same tick pass through each other undetected).
            for i in 0..self.carts.len() {
                self.carts[i].next_cart_position(&self.map);
                let pos = self.carts[i].position;

                let mut response: Option<String> = None;
                if self
                    .carts
                    .iter()
                    .enumerate()
                    .any(|(j, other)| j != i && other.position == pos)
                {
                    response = Some(format!("{},{}", pos.x(), pos.y()));
                }

                if response.is_some() {
                    return response;
                }
            }
        }
    }
    pub fn simulate_part2(&mut self) -> Option<String> {
        loop {
            // Carts tick in reading order (top-to-bottom, left-to-right).
            self.carts
                .sort_unstable_by_key(|cart| (cart.position.y(), cart.position.x()));

            // Move one cart at a time; a collision is detected the instant a
            // cart steps onto another cart's current cell. Checking only after
            // every cart has moved would miss head-on swaps (two carts trading
            // cells in the same tick pass through each other undetected).
            let mut collided_indices = Vec::new();
            for i in 0..self.carts.len() {
                self.carts[i].next_cart_position(&self.map);
                let pos = self.carts[i].position;

                if let Some((j, _)) = self
                    .carts
                    .iter()
                    .enumerate()
                    .find(|(j, other)| *j != i && other.position == pos)
                {
                    collided_indices.push(i);
                    collided_indices.push(j);
                }
            }

            let mut i = 0;
            self.carts.retain(|_| {
                let keep = !collided_indices.contains(&i);
                i += 1;
                keep
            });

            if self.carts.len() == 1 {
                let position = self.carts.get(0).unwrap().position;
                return Some(format!("{},{}", position.x(), position.y()));
            }
        }
    }

    pub fn print_grid(&self) {
        let mut display: HashMap<Coord<usize>, char> = HashMap::new();
        self.map.iter().for_each(|(coord, map_type)| {
            let c = match map_type {
                Vertical => '|',
                LeftToBottom => '\\',
                LeftToTop => '/',
                Intersection => '+',
                Horizontal => '-',
                Space => ' ',
            };
            display.entry(*coord).or_insert(c);
        });
        self.carts.iter().for_each(|cart| {
            let c = cart.direction.as_char();
            *display.entry(cart.position).or_insert(c) = c;
        });

        let x_max = display.keys().map(|k| k.x()).max().unwrap();
        let y_max = display.keys().map(|k| k.y()).max().unwrap();

        let mut message = String::new();
        (0..=y_max).for_each(|y| {
            (0..=x_max).for_each(|x| {
                let coord = Coord::new(x, y);
                let c = display.get(&coord).unwrap_or(&' ');
                message.push(*c);
            });
            message.push('\n');
        });

        println!("{message}");
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let mut state: GridState = input.parse().unwrap();
    state.simulate_part1()
}

pub fn part_two(input: &str) -> Option<String> {
    let mut state: GridState = input.parse().unwrap();
    state.simulate_part2()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(String::from("7,3")));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(String::from("6,4")));
    }

    #[test]
    fn test_carts() {
        let mut direction = CartDirectionChoice::Left;

        assert_eq!(direction.next(), CartDirectionChoice::Left);
        assert_eq!(direction.next(), CartDirectionChoice::Straight);
        assert_eq!(direction.next(), CartDirectionChoice::Right);
        assert_eq!(direction.next(), CartDirectionChoice::Left);
    }

    #[test]
    fn test_head_on_swap() {
        // Two carts 5 cells apart (odd gap): under simultaneous movement they
        // swap cells and pass through each other undetected. Correct behavior:
        // the right-mover reaches x=3 (reading order) and collides there.
        let mut state: GridState = ">----<".parse().unwrap();
        assert_eq!(state.simulate_part1(), Some(String::from("3,0")));
    }

    #[test]
    fn test_mapping() {
        assert_eq!(MapType::from_char('/').unwrap(), LeftToTop);
        assert_eq!(MapType::from_char('+').unwrap(), Intersection);
        assert_eq!(MapType::from_char('\\').unwrap(), LeftToBottom);
        assert_eq!(MapType::from_char(' ').unwrap(), Space);
        assert_eq!(MapType::from_char('|').unwrap(), Vertical);
        assert_eq!(MapType::from_char('v').unwrap(), Vertical);
        assert_eq!(MapType::from_char('^').unwrap(), Vertical);
        assert_eq!(MapType::from_char('-').unwrap(), Horizontal);
        assert_eq!(MapType::from_char('>').unwrap(), Horizontal);
        assert_eq!(MapType::from_char('<').unwrap(), Horizontal);
    }
}
