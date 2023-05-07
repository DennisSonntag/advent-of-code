use std::collections::BTreeSet;

pub fn process_part1(input: &str) -> usize {
	input
		.chars()
		.collect::<Vec<char>>()
		.windows(4)
		.enumerate()
		.find(|(_, slice)| {
			let set = slice.iter().collect::<BTreeSet<&char>>();
			slice.len() == set.len()
		})
		.unwrap()
		.0 + 1 + 3
}

pub fn process_part2(input: &str) -> usize {
	input
		.chars()
		.collect::<Vec<char>>()
		.windows(14)
		.enumerate()
		.find(|(_, slice)| {
			let set = slice.iter().collect::<BTreeSet<&char>>();
			slice.len() == set.len()
		})
		.unwrap()
		.0 + 1 + 13

}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	 #[ignore]
	fn part1() {
		assert_eq!(process_part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
		assert_eq!(process_part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
		assert_eq!(process_part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
		assert_eq!(process_part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
		assert_eq!(process_part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
	}

	#[test]
	// #[ignore]
	fn part2() {
		assert_eq!(process_part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
		assert_eq!(process_part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
		assert_eq!(process_part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
		assert_eq!(process_part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
		assert_eq!(process_part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
	}

}
