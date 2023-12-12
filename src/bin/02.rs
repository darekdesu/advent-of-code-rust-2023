advent_of_code::solution!(2);
const RED_CUBES_LIMIT: u32 = 12;
const GREEN_CUBES_LIMIT: u32 = 13;
const BLUE_CUBES_LIMIT: u32 = 14;

pub fn is_limit_reached(words: Vec<&str>, limit: u32) -> bool {
    words.get(0).unwrap_or(&"0").parse().unwrap_or(0) > limit
}

pub fn get_number_of_cubes(words: Vec<&str>) -> u32 {
    words.get(0).unwrap_or(&"0").parse().unwrap_or(0)
}
pub fn part_one(input: &str) -> Option<u32> {
    let result =
        input
            .lines()
            .map(|line| {
                let mut game_number: u32 = 0;
                let mut red_counter: Vec<bool> = Vec::new();
                let mut green_counter: Vec<bool> = Vec::new();
                let mut blue_counter: Vec<bool> = Vec::new();

                line.split(|c| c == ':' || c == ';' || c == ',')
                    .for_each(|s| {
                        let game_with_outputs: Vec<&str> = s.trim().split_whitespace().collect();

                        match s {
                            s if s.contains("Game") => {
                                game_number = game_with_outputs
                                    .get(1)
                                    .unwrap_or(&"0")
                                    .parse()
                                    .unwrap_or(0)
                            }
                            s if s.contains("red") => red_counter
                                .push(is_limit_reached(game_with_outputs, RED_CUBES_LIMIT)),
                            s if s.contains("green") => green_counter
                                .push(is_limit_reached(game_with_outputs, GREEN_CUBES_LIMIT)),
                            s if s.contains("blue") => blue_counter
                                .push(is_limit_reached(game_with_outputs, BLUE_CUBES_LIMIT)),
                            _ => {}
                        }
                    });

                if red_counter.iter().find(|&&x| x == true).is_some()
                    || green_counter.iter().find(|&&x| x == true).is_some()
                    || blue_counter.iter().find(|&&x| x == true).is_some()
                {
                    return 0;
                }

                game_number
            })
            .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(|line| {
            let mut game_number: u32 = 0;
            let mut red_counter: Vec<u32> = Vec::new();
            let mut green_counter: Vec<u32> = Vec::new();
            let mut blue_counter: Vec<u32> = Vec::new();

            line.split(|c| c == ':' || c == ';' || c == ',')
                .for_each(|s| {
                    let game_with_outputs: Vec<&str> = s.trim().split_whitespace().collect();

                    match s {
                        s if s.contains("Game") => {
                            game_number = game_with_outputs
                                .get(1)
                                .unwrap_or(&"0")
                                .parse()
                                .unwrap_or(0)
                        }
                        s if s.contains("red") => {
                            red_counter.push(get_number_of_cubes(game_with_outputs))
                        }
                        s if s.contains("green") => {
                            green_counter.push(get_number_of_cubes(game_with_outputs))
                        }
                        s if s.contains("blue") => {
                            blue_counter.push(get_number_of_cubes(game_with_outputs))
                        }
                        _ => {}
                    }
                });

            let minimum_red_counter = red_counter.iter().max().unwrap_or(&0);
            let minimum_green_counter = green_counter.iter().max().unwrap_or(&0);
            let minimum_blue_counter = blue_counter.iter().max().unwrap_or(&0);

            minimum_red_counter * minimum_green_counter * minimum_blue_counter
        })
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
