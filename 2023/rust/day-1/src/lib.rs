#![allow(unused, dead_code, clippy::panic)]

pub fn process_part1(input: &str) -> u32 {
	input
		.lines()
		.map(|line| {
			let nums = line.chars().filter(char::is_ascii_digit).collect::<Vec<_>>();
			format!("{}{}", nums.first().unwrap(), nums.last().unwrap()).parse::<u32>().unwrap()
		})
		.sum()
}

pub fn process_part2(input: &str) -> u32 {
	input.lines().map(|line| {
		let mut i = 0;
		let reduces_line = &line[i..];
		let itr = std::iter::from_fn(move || {
			i += 1;
			// Check to see if we've finished counting or not.
			if i < line.len() {


			} else {
				None
			}
		});
	});

	41
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
