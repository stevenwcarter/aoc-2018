advent_of_code::solution!(8);

pub struct Node {
    pub child_node_count: u8,
    pub metadata_count: u8,
    pub child_nodes: Vec<Node>,
    pub metadata_entries: Vec<u8>,
}

impl Node {
    pub fn parse(input: &[u8]) -> (Option<Self>, &[u8]) {
        if input.is_empty() {
            return (None, input);
        }
        let child_node_count = input[0];
        let metadata_count = input[1];
        let mut child_nodes = Vec::new();
        let mut metadata_entries = Vec::new();

        let mut remainder = &input[2..];

        if child_node_count > 0 {
            for _ in 0..child_node_count {
                let (node, new_remainder) = Node::parse(remainder);
                child_nodes.push(node.unwrap());
                remainder = new_remainder;
            }
        }

        if metadata_count > 0 {
            metadata_entries = remainder[0..metadata_count as usize].to_vec();
            remainder = &remainder[metadata_count as usize..];
        }

        (
            Some(Self {
                child_node_count,
                metadata_count,
                child_nodes,
                metadata_entries,
            }),
            remainder,
        )
    }

    pub fn metadata_sum(&self) -> u32 {
        self.metadata_entries.iter().map(|&n| n as u32).sum::<u32>()
            + self
                .child_nodes
                .iter()
                .map(|cn| cn.metadata_sum())
                .sum::<u32>()
    }

    pub fn node_value(&self) -> u32 {
        if self.child_nodes.is_empty() {
            self.metadata_entries.iter().map(|&n| n as u32).sum::<u32>()
        } else {
            self.metadata_entries
                .iter()
                .filter_map(|&m| {
                    if m == 0 {
                        None
                    } else {
                        self.child_nodes.get((m - 1) as usize)
                    }
                })
                .map(|n| n.node_value())
                .sum()
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let data: Vec<u8> = input
        .trim()
        .split(' ')
        .map(|ch| {
            ch.parse::<u8>()
                .unwrap_or_else(|_| panic!("could not convert '{ch}'"))
        })
        .collect();
    let (top_node, _) = Node::parse(&data);

    Some(top_node.unwrap().metadata_sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let data: Vec<u8> = input
        .trim()
        .split(' ')
        .map(|ch| {
            ch.parse::<u8>()
                .unwrap_or_else(|_| panic!("could not convert '{ch}'"))
        })
        .collect();
    let (top_node, _) = Node::parse(&data);

    Some(top_node.unwrap().node_value())
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
