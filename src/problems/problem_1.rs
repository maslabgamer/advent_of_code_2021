fn count_increases(input: &Vec<i32>) -> u32 {
    let mut increases = 0;
    let mut previous = input[0];
    for i in 1..input.len() {
        let current = input[i];
        increases += (current > previous) as u32;
        previous = current;
    }
    increases
}

fn three_measurement_sliding_window(input: &Vec<i32>) -> u32 {
    let mut increases = 0;
    let mut previous = input[0] + input[1] + input[2];
    for i in 3..input.len() {
        let current = input[i] + input[i - 1] + input[i - 2];
        increases += (current > previous) as u32;
        previous = current;
    }
    increases
}

#[cfg(test)]
mod tests {
    use crate::problems::problem_1::{count_increases, three_measurement_sliding_window};

    #[test]
    fn part_one() {
        let input: Vec<i32> = include_str!("../../resources/problem_1_input.txt").lines()
            .map(|number| number.parse::<i32>().unwrap())
            .collect();
        assert_eq!(1832, count_increases(&input));
    }

    #[test]
    fn part_two() {
        let input: Vec<i32> = include_str!("../../resources/problem_1_part_two_input.txt").lines()
            .map(|number| number.parse::<i32>().unwrap())
            .collect();

        println!("{}", three_measurement_sliding_window(&input));
    }
}