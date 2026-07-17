use hashbrown::HashMap;

advent_of_code::solution!(4);

enum Event {
    BeginShift(u32),
    FallAsleep,
    WakeUp,
}

struct LogEntry {
    minute: usize,
    event: Event,
}

fn parse(input: &str) -> Vec<LogEntry> {
    let mut lines: Vec<&str> = input.lines().filter(|l| !l.trim().is_empty()).collect();
    lines.sort_unstable();

    lines
        .iter()
        .map(|line| {
            let minute = line[15..17].parse().expect("minute should be two digits");

            let event = if let Some((_, rest)) = line.split_once("Guard #") {
                let id = rest
                    .split_whitespace()
                    .next()
                    .and_then(|tok| tok.parse().ok())
                    .expect("guard line should contain an id");
                Event::BeginShift(id)
            } else if line.contains("falls asleep") {
                Event::FallAsleep
            } else {
                Event::WakeUp
            };

            LogEntry { minute, event }
        })
        .collect()
}

fn sleep_histograms(entries: &[LogEntry]) -> HashMap<u32, [u32; 60]> {
    let mut histograms: HashMap<u32, [u32; 60]> = HashMap::new();
    let mut current_guard = 0;
    let mut asleep_since = 0;

    for entry in entries {
        match entry.event {
            Event::BeginShift(id) => current_guard = id,
            Event::FallAsleep => asleep_since = entry.minute,
            Event::WakeUp => {
                let histogram = histograms.entry(current_guard).or_insert([0; 60]);
                for count in histogram.iter_mut().take(entry.minute).skip(asleep_since) {
                    *count += 1;
                }
            }
        }
    }

    histograms
}

pub fn part_one(input: &str) -> Option<u64> {
    let histograms = sleep_histograms(&parse(input));

    let (&id, histogram) = histograms
        .iter()
        .max_by_key(|(_, histogram)| histogram.iter().sum::<u32>())?;

    let (best_minute, _) = histogram
        .iter()
        .enumerate()
        .max_by_key(|(_, &count)| count)?;

    Some(id as u64 * best_minute as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let histograms = sleep_histograms(&parse(input));

    let (id, minute, _) = histograms
        .iter()
        .flat_map(|(&id, histogram)| {
            histogram
                .iter()
                .enumerate()
                .map(move |(minute, &count)| (id, minute, count))
        })
        .max_by_key(|&(_, _, count)| count)?;

    Some(id as u64 * minute as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(240));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4455));
    }
}
