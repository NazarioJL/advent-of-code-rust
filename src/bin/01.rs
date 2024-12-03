use std::iter::zip;
use counter::Counter;

advent_of_code::solution!(1);


fn get_input_as_tuples(input: &str) -> Vec<(u32,u32)>{
    input.lines().map(|line| {
        let mut nums = line.split_ascii_whitespace();
        let left: u32 = nums.next().expect("There should be a first element").parse().expect("Not a string");
        let right: u32 = nums.next().expect("There should be a seconds element").parse().expect("Not a string");
        (left, right)
    }).collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let values: &Vec<(u32, u32)> = &get_input_as_tuples(input);
    let mut left_list: Vec<u32> = values.into_iter().map(|item| {item.0}).collect();
    let mut right_list: Vec<u32> = values.into_iter().map(|item| {item.1}).collect();

    left_list.sort();
    right_list.sort();

    Some(zip(left_list, right_list).map(|e| e.0.abs_diff(e.1)).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let values = &get_input_as_tuples(input);
    let freq = values.into_iter().map(|item| item.1).collect::<Counter<u32, u32>>();

    Some(
        values
        .into_iter()
        .map(|e| (e.0 * freq[&e.0]))
        .sum::<u32>() // Why do I have to do this? :'(
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
