advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let total_sum = input
        .lines()
        .map(|line| {
            let parsed_digits = parse_digits(&line);
            let first_digit = parsed_digits.first().unwrap();
            let last_digit = parsed_digits.last().unwrap();
            let sum: u32 = format!("{:?}{:?}", first_digit, last_digit)
                .parse::<u32>()
                .unwrap();
            sum
        })
        .sum();

    Some(total_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let total_sum = input
        .lines()
        .map(|line| {
            let spelled_letters = [
                "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
            ];
            let mut temporary_line_chars = String::new();
            let mut new_line = String::from(line);
            for c in line.chars() {
                // create and add each character to new temporary chars variable
                temporary_line_chars.push(c);

                // if is number then continue (and reset temporary chars variable)
                if c.is_digit(10) {
                    temporary_line_chars.clear();
                    continue;
                }

                // if substring contains item from array of spelled numbers add it to found spelled numbers array
                let found_spelled_index = spelled_letters
                    .iter()
                    .position(|&item| temporary_line_chars.contains(item));

                match found_spelled_index {
                    Some(i) => {
                        new_line =
                            new_line.replacen(spelled_letters[i], (i + 1).to_string().as_str(), 1);
                        temporary_line_chars.clear();
                        continue;
                    }
                    None => continue,
                }
            }

            let parsed_digits = parse_digits(&new_line);
            let first_digit = parsed_digits.first().unwrap();
            let last_digit = parsed_digits.last().unwrap();
            let sum: u32 = format!("{:?}{:?}", first_digit, last_digit)
                .parse::<u32>()
                .unwrap();
            sum
        })
        .sum();

    Some(total_sum)
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
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}

fn parse_digits(t_num: &str) -> Vec<u32> {
    t_num.chars().filter_map(|a| a.to_digit(10)).collect()
}
