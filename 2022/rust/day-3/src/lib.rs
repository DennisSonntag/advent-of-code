#![feature(iter_array_chunks)]
use std::collections::HashMap;

pub fn process_part1(input: &str) -> usize {
	let letter_priotities = ('a'..='z')
		.chain('A'..='Z')
		.enumerate()
		.map(|(idx, char)| (char, idx + 1))
		.collect::<HashMap<char, usize>>();

	input
		.lines()
		.map(|line| {
			let (compartment_a, compartment_b) = line.split_at(line.len() / 2);

			let common_char = compartment_a
				.chars()
				.find(|c| compartment_b.contains(*c))
				.unwrap();
			letter_priotities.get(&common_char).unwrap()
		})
		.sum()
}

pub fn process_part2(input: &str) -> usize {
	let letter_priotities = ('a'..='z')
		.chain('A'..='Z')
		.enumerate()
		.map(|(idx, char)| (char, idx + 1))
		.collect::<HashMap<char, usize>>();

	input
		.lines()
		.array_chunks::<3>()
		.map(|[a, b, c]| {
			let common_char = a
				.chars()
				.find(|char| b.contains(*char) && c.contains(*char))
				.unwrap();
			letter_priotities.get(&common_char).unwrap()
		})
		.sum()
}

#[cfg(test)]
mod tests {
	use super::*;

	const TEST_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

	#[test]
	fn part1() {
		let result = process_part1(TEST_INPUT);
		assert_eq!(result, 157);
	}

	#[test]
	#[ignore]
	fn part2() {
		let result = process_part2(TEST_INPUT);
		assert_eq!(result, 70);
	}
}
