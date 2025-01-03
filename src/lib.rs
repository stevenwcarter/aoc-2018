use hashbrown::HashMap;

pub mod template;

// Use this file to add helper functions and additional modules.

// helper for day 2
pub fn check_for_repeats(input: &str) -> (bool, bool) {
    let mut counts: HashMap<char, u8> = HashMap::new();
    input
        .chars()
        .for_each(|c| *counts.entry(c).or_insert(0) += 1);

    (
        counts.values().any(|&c| c == 2),
        counts.values().any(|&c| c == 3),
    )
}
pub fn check_for_repeats_vec(input: &str) -> (bool, bool) {
    let mut counts = [0; 26];

    input
        .chars()
        .for_each(|c| counts[(c as u8 - b'a') as usize] += 1);

    (
        counts.iter().any(|&c| c == 2),
        counts.iter().any(|&c| c == 3),
    )
}

#[cfg(test)]
mod tests {
    use crate::check_for_repeats;

    use super::*;

    #[test]
    fn it_check_for_repeats() {
        assert_eq!(check_for_repeats("abcdde"), (true, false));
        assert_eq!(check_for_repeats("abcddefefe"), (true, true));
    }
    #[test]
    fn it_check_for_repeats_vec() {
        assert_eq!(check_for_repeats_vec("abcdde"), (true, false));
        assert_eq!(check_for_repeats_vec("abcddefefe"), (true, true));
    }
}
