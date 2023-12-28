use regex::Regex;
use std::collections::HashSet;

advent_of_code::solution!(4);

fn get_matching_numbers(input: &str) -> Vec<usize> {
    let re = Regex::new(r#"\d+"#).unwrap();

    input
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
        .collect::<Vec<usize>>()
}

pub fn part_one(input: &str) -> Option<u32> {
    let result = get_matching_numbers(input);

    Some(
        result
            .iter()
            .filter(|x| x.ne(&&0))
            .map(|x| 2u32.pow((x - 1) as u32))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let matching_numbers = get_matching_numbers(input);
    let mut scratchcards_counter = vec![1; matching_numbers.len()];

    for (idx, matching_number) in matching_numbers.iter().enumerate() {
        let won_scratchcards_copies = matching_numbers
            .iter()
            .skip(idx)
            .take(*matching_number)
            .count();

        let current_scratchcard_counter = scratchcards_counter[idx];
        let next_scratchcards_indexes = idx + 1..=idx + won_scratchcards_copies;
        for next_scratchcard_idx in next_scratchcards_indexes {
            scratchcards_counter[next_scratchcard_idx] += current_scratchcard_counter;
        }
    }

    Some(scratchcards_counter.iter().sum())
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
        assert_eq!(result, Some(30));
    }
}
