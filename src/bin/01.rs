use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex},
};

advent_of_code::solution!(1);

static NUMS_LITERAL: LazyLock<Mutex<HashMap<String, u32>>> =
    LazyLock::new(|| Mutex::new((0..=9).map(|e| (e.to_string(), e)).collect()));

static NUMS_STR: LazyLock<Mutex<HashMap<String, u32>>> = LazyLock::new(|| {
    Mutex::new(HashMap::from([
        (String::from("one"), 1),
        (String::from("two"), 2),
        (String::from("three"), 3),
        (String::from("four"), 4),
        (String::from("five"), 5),
        (String::from("six"), 6),
        (String::from("seven"), 7),
        (String::from("eight"), 8),
        (String::from("nine"), 9),
    ]))
});

pub fn get_number_value(line: &str, lookup: &HashMap<String, u32>) -> u32 {
    let locations: Vec<(usize, u32)> = lookup
        .keys()
        .map(|f| (line.find(f), lookup.get(f)))
        .filter(|e| e.0.is_some() && e.1.is_some())
        .map(|e| (e.0.expect("None is unexpected"), *e.1.expect("None is unexpected")))
        .collect();

    let first: u32 = locations
        .iter()
        .min_by_key(|f| f.0)
        .expect("First matching substring not found")
        .1;

    let last: u32 = locations
        .iter()
        .max_by_key(|f| f.0)
        .expect("Last matching substring not found")
        .1;

    first * 10 + last
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = NUMS_LITERAL.lock().unwrap();
    Some(input.lines().map(|line| get_number_value(line, &map)).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    // Combine NUMS_LITERAL with NUMS_STR
    let mut combined = NUMS_LITERAL.lock().unwrap().clone();
    combined.extend(NUMS_STR.lock().unwrap().clone());

    Some(
        input
            .lines()
            .map(|line| get_number_value(line, &combined))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
