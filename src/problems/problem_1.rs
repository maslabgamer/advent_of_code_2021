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

#[cfg(test)]
mod tests {
    use crate::problems::problem_1::count_increases;

    #[test]
    fn part_one() {
        let input: Vec<i32> = include_str!("../../resources/problem_1_input.txt").lines()
            .map(|number| number.parse::<i32>().unwrap())
            .collect();
        assert_eq!(1832, count_increases(&input));
    }
}