use regex::Regex;
use std::collections::HashSet;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r#"\d+"#).unwrap();

    let result = input
        .lines()
        .flat_map(|line| {
            line.split(|c| c == ':')
                .filter(|s| !s.contains("Card"))
                .map(|s| {
                    let numbers: Vec<u32> = re
                        .find_iter(s)
                        .map(|number| number.as_str().parse::<u32>().unwrap_or(0))
                        .collect();
                    let unique_numbers: HashSet<u32> = numbers.clone().into_iter().collect();

                    numbers.len() - unique_numbers.len()
                })
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<usize>>();

    Some(
        result
            .iter()
            .filter(|x| x.ne(&&0))
            .map(|x| 2u32.pow((x - 1) as u32))
            .sum(),
    )
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
