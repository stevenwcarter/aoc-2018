use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space1},
    combinator::map_res,
    sequence::{preceded, tuple},
    IResult,
};

advent_of_code::solution!(3);

pub struct Grid([u8; 1000000]);

impl Default for Grid {
    fn default() -> Self {
        Self([0; 1000000])
    }
}

impl Grid {
    pub fn process_claim(&mut self, claim: Claim) {
        self.mark_ranges(claim.offset_l, claim.offset_t, claim.width, claim.height)
    }
    pub fn mark_ranges(&mut self, left_offset: u16, top_offset: u16, width: u16, height: u16) {
        for x in left_offset..left_offset + width {
            for y in top_offset..top_offset + height {
                let position = (y as usize * 1000) + x as usize;
                self.0[position] += 1;
            }
        }
    }

    pub fn count_overlaps(&self) -> Option<usize> {
        Some(self.0.iter().filter(|&&l| l > 1).count())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Claim {
    pub id: u16,
    pub offset_l: u16,
    pub offset_t: u16,
    pub width: u16,
    pub height: u16,
}
impl Claim {
    pub fn parse(input: &str) -> Self {
        parse_claim(input).unwrap().1
    }

    pub fn left(&self) -> u16 {
        self.offset_l
    }
    pub fn right(&self) -> u16 {
        self.offset_l + self.width - 1
    }
    pub fn bottom(&self) -> u16 {
        self.offset_t + self.height - 1
    }
    pub fn top(&self) -> u16 {
        self.offset_t
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        !(self.right() < other.left()
            || self.left() > other.right()
            || self.bottom() < other.top()
            || self.top() > other.bottom())
    }
}

fn parse_u16(input: &str) -> IResult<&str, u16> {
    map_res(digit1, |digit_str: &str| digit_str.parse::<u16>())(input)
}

fn parse_claim(input: &str) -> IResult<&str, Claim> {
    let (input, (_, id, _, offset_l, _, offset_t, _, width, _, height)) = tuple((
        tag("#"),
        parse_u16,
        space1,
        preceded(tag("@ "), parse_u16),
        tag(","),
        parse_u16,
        tag(": "),
        parse_u16,
        tag("x"),
        parse_u16,
    ))(input)?;

    Ok((
        input,
        Claim {
            id,
            offset_l,
            offset_t,
            width,
            height,
        },
    ))
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut grid = Grid::default();
    input
        .lines()
        .map(Claim::parse)
        .for_each(|c| grid.process_claim(c));
    grid.count_overlaps()
}

fn find_non_overlapping_optimized(claims: &[Claim]) -> Option<&Claim> {
    let mut sorted_claims: Vec<&Claim> = claims.iter().collect();
    sorted_claims.sort_by_key(|claim| claim.left());
    let mut iter_count = 0;

    sorted_claims
        .iter()
        .find(|&&claim| {
            iter_count += 1;
            !sorted_claims
                .iter()
                .any(|&&other| *claim != other && claim.overlaps(&other))
        })
        .cloned()
}

pub fn part_two(input: &str) -> Option<u16> {
    let claims: Vec<Claim> = input.lines().map(Claim::parse).collect();
    let result = find_non_overlapping_optimized(&claims).expect("Could not solve");

    Some(result.id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }
}
