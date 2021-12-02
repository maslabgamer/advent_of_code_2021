fn process_sub_commands(input: &Vec<&str>) -> (i32, i32) {
    let mut forward_amount = 0;
    let mut depth = 0;

    for line in input {
        let mut line = line.split_ascii_whitespace();
        let command = line.next().unwrap();
        let amount = line.next().unwrap().parse::<i32>().unwrap();
        match command {
            "forward" => forward_amount += amount,
            "down" => depth += amount,
            "up" => depth -= amount,
            _ => panic!("Invalid command")
        }
    }

    (forward_amount, depth)
}

fn process_sub_commands_account_for_aim(input: &Vec<&str>) -> (i32, i32) {
    let mut forward_amount = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in input {
        let mut line = line.split_ascii_whitespace();
        let command = line.next().unwrap();
        let amount = line.next().unwrap().parse::<i32>().unwrap();
        match command {
            "forward" => {
                forward_amount += amount;
                depth += aim * amount;
            },
            "down" => aim += amount,
            "up" => aim -= amount,
            _ => panic!("Invalid command")
        }
    }

    (forward_amount, depth)
}

#[cfg(test)]
mod tests {
    use crate::problems::problem_2::{process_sub_commands, process_sub_commands_account_for_aim};

    #[test]
    fn part_one() {
        let input: Vec<&str> = include_str!("../../resources/problem_2_input.txt").lines()
            .collect();
        let (distance, amount) = process_sub_commands(&input);
        assert_eq!(1670340, distance * amount);
    }

    #[test]
    fn part_two() {
        let input: Vec<&str> = include_str!("../../resources/problem_2_input.txt").lines()
            .collect();
        let (distance, amount) = process_sub_commands_account_for_aim(&input);
        assert_eq!(1954293920, distance * amount);
    }
}