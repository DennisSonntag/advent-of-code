#![allow(unused)]
use std::collections::HashMap;

pub fn process_part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let nums = line
                .chars()
                .filter(|x| x.is_ascii_digit())
                .collect::<Vec<_>>();
            let first = nums.first().unwrap();
            let last = nums.last().unwrap();
            let num = format!("{first}{last}").parse::<u32>().unwrap();
            return num;
        })
        .sum()
}

pub fn process_part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let line = line
                .replace("one", 1)
                .replace("two", 2)
                .replace("three", 3)
                .replace("four", 4)
                .replace("five", 5)
                .replace("six", 6)
                .replace("seven", 7)
                .replace("eight", 8)
                .replace("nine", 9);
            let nums = line
                .chars()
                .filter(|x| x.is_ascii_digit())
                .collect::<Vec<_>>();
            let first = nums.first().unwrap();
            let last = nums.last().unwrap();
            let num = format!("{first}{last}").parse::<u32>().unwrap();
            return num;
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
