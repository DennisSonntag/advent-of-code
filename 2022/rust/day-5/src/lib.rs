#![allow(unused)]
use std::collections::HashMap;

pub fn process_part1(input: &str) -> String {
	let stacks: HashMap<i32, Vec<char>> = HashMap::new();

	let (inp_stacks, instructions) = input.split_once("\n\n").unwrap();
	let width = inp_stacks.split_once("\n").unwrap().0.len();
	let col1 = inp_stacks
		.lines()
		.map(|x| {
			x.chars()
				.enumerate()
				.filter(|(idx, x)| idx == &1)
				.map(|(idx, x)| x)
				.filter(|x| x != &'\n' && x != &' ')
				// .filter(|x| x != &' ')
				.collect::<String>()
		})
		.collect::<Vec<_>>();
	// println!("{:?}", col1);
	// println!("{}", inp_stacks);
	println!("{}", width);

	"idk".to_string()
}

pub fn process_part2(input: &str) -> usize {
	unimplemented!()
}

#[cfg(test)]
mod tests {
	use super::*;

	const TEST_INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

	#[test]
	// #[ignore]
	fn part1() {
		let result = process_part1(TEST_INPUT);
		assert_eq!(result, "CMZ");
	}

	#[test]
	#[ignore]
	fn part2() {
		let result = process_part2(TEST_INPUT);
		assert_eq!(result, 4);
	}
}
