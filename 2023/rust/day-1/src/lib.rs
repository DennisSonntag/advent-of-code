#![allow(unused, dead_code, clippy::panic)]

pub fn process_part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let nums = line
                .chars()
                .filter(char::is_ascii_digit)
                .collect::<Vec<_>>();
            format!("{}{}", nums.first().unwrap(), nums.last().unwrap())
                .parse::<u32>()
                .unwrap()
        })
        .sum()
}

pub fn process_part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut it = (0..line.len()).filter_map(|index| {
                let reduced_line = &line[index..];
                let result = if reduced_line.starts_with("one") {
                    '1'
                } else if reduced_line.starts_with("two") {
                    '2'
                } else if reduced_line.starts_with("three") {
                    '3'
                } else if reduced_line.starts_with("four") {
                    '4'
                } else if reduced_line.starts_with("five") {
                    '5'
                } else if reduced_line.starts_with("six") {
                    '6'
                } else if reduced_line.starts_with("seven") {
                    '7'
                } else if reduced_line.starts_with("eight") {
                    '8'
                } else if reduced_line.starts_with("nine") {
                    '9'
                } else {
                    reduced_line.chars().next().unwrap()
                };

                result.to_digit(10)
            });
            let first = it.next().expect("should be a number");

            match it.last() {
                Some(num) => format!("{first}{num}"),
                None => format!("{first}{first}"),
            }
            .parse::<u32>()
            .expect("should be a valid number")
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    const TEST_INPUT2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    #[ignore]
    fn part1_works() {
        let result = process_part1(TEST_INPUT);
        assert_eq!(result, 142);
    }

    #[test]
    fn part2_works() {
        let result = process_part2(TEST_INPUT2);
        assert_eq!(result, 281);
    }
}
