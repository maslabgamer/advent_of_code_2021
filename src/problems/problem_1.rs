fn window_increase(input: &Vec<i32>, window_size: usize) -> u32 {
    let mut increases = 0;
    let mut previous: i32 = (input[0..window_size]).iter().sum();

    for i in window_size..input.len() {
        let current: i32 = (input[i - (window_size - 1)..=i]).iter().sum();
        increases += (current > previous) as u32;
        previous = current;
    }

    increases
}

#[cfg(test)]
mod tests {
    use crate::problems::problem_1::window_increase;

    #[test]
    fn part_one() {
        let input: Vec<i32> = include_str!("../../resources/problem_1_input.txt").lines()
            .map(|number| number.parse::<i32>().unwrap())
            .collect();
        assert_eq!(1832, window_increase(&input, 1));
    }

    #[test]
    fn part_two() {
        let input: Vec<i32> = include_str!("../../resources/problem_1_part_two_input.txt").lines()
            .map(|number| number.parse::<i32>().unwrap())
            .collect();
        assert_eq!(1858, window_increase(&input, 3));
    }
}