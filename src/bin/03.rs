advent_of_code::solution!(3);

pub fn is_engine_number(value: char) -> bool {
    value != '.' && value != 'X'
}

pub fn part_one(input: &str) -> Option<u32> {
    let size = input.lines().count() + 2;
    let mut grid = vec![vec!['.'; size]; size];

    input.lines().enumerate().for_each(|(col_idx, line)| {
        for (row_idx, char) in line.chars().enumerate() {
            if char.is_ascii_digit() {
                grid[col_idx + 1][row_idx + 1] = char;
                continue;
            }

            if char != '.' {
                grid[col_idx + 1][row_idx + 1] = 'X';
            }
        }
    });

    let neighbors: [[isize; 2]; 8] = [
        [-1, -1],
        [-1, 0],
        [-1, 1],
        [0, -1],
        [0, 1],
        [1, -1],
        [1, 0],
        [1, 1],
    ];

    let mut found_values_list = Vec::new();

    for i in 1..size - 1 {
        let mut temporary_value = String::new();
        let mut is_current_temporary_eligible = false;
        for j in 1..size - 1 {
            let current = grid[i][j];
            let next = grid[i][j + 1];
            let is_last = j == (size - 2);

            if is_engine_number(current) {
                temporary_value.push(current);

                neighbors.iter().for_each(|[x, y]| {
                    let new_i = i as isize + x;
                    let new_j = j as isize + y;
                    let neighbor = grid[new_i as usize][new_j as usize];

                    if neighbor == 'X' {
                        is_current_temporary_eligible = true;
                    }
                });
            } else if !temporary_value.is_empty() && is_current_temporary_eligible {
                found_values_list.push(temporary_value.clone());
                is_current_temporary_eligible = false;
                temporary_value.clear();
            }

            if next == '.' && !temporary_value.is_empty() && !is_current_temporary_eligible {
                temporary_value.clear();
            }

            if is_last && !temporary_value.is_empty() && is_current_temporary_eligible {
                found_values_list.push(temporary_value.clone());
                is_current_temporary_eligible = false;
                temporary_value.clear();
            }
        }
    }

    let result = found_values_list
        .iter()
        .map(|s| s.parse::<u32>().unwrap_or(0))
        .sum();

    Some(result)
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
        assert_eq!(result, Some(925));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}