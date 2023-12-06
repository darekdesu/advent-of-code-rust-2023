advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut total_sum = 0;

    for (index, line) in input.lines().enumerate() {
        let parsed_digits = parse_digits(&line);
        let first_digit = parsed_digits.first().unwrap();
        let last_digit = parsed_digits.last().unwrap();
        let sum = format!("{:?}{:?}", first_digit, last_digit)
            .parse::<u32>()
            .unwrap();
        total_sum += sum;

        println!("{}. {} = {}{}", index + 1, line, first_digit, last_digit);
    }

    Some(total_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

fn parse_digits(t_num: &str) -> Vec<u32> {
    t_num.chars().filter_map(|a| a.to_digit(10)).collect()
}