use std::collections::BTreeMap;

pub fn process_part1(input: &str) -> String {
	let mut stacks: BTreeMap<u32, Vec<char>> = BTreeMap::new();

	let (inp_stacks, instructions) = input.split_once("\n\n").unwrap();
	let width = inp_stacks.split_once('\n').unwrap().0.len();

	for i in (1..=(((width + 1) / 4).pow(2))).step_by(4) {
		let col = inp_stacks
			.lines()
			.map(|line| line.chars().nth(i).unwrap_or(' '))
			.filter(|x| !x.is_whitespace())
			.collect::<Vec<_>>();
		if !col.is_empty() {
			let (num, col) = col.split_last().unwrap();
			let num = num.to_digit(10).unwrap();
			let mut col = col.to_vec();
			col.reverse();
			stacks.insert(num, col);
		}
	}

	let instructions: Vec<_> = instructions
		.lines()
		.map(|line| {
			let mut parts = line
				.split_whitespace()
				.filter_map(|x| x.parse::<i32>().ok());
			(
				parts.next().unwrap(),
				parts.next().unwrap(),
				parts.next().unwrap(),
			)
		})
		.collect();

	for (amount, from, to) in instructions {
		if let Some(vec) = stacks.get_mut(&(from as u32)) {
			let mut retrived_items = vec.drain(vec.len() - amount as usize..).rev().collect();
			if let Some(vec) = stacks.get_mut(&(to as u32)) {
				vec.append(&mut retrived_items);
			}
		}
	}

	let answer = stacks
		.iter()
		.map(|(_, val)| val.last().unwrap())
		.collect::<String>();

	answer
}

pub fn process_part2(input: &str) -> String {
	let mut stacks: BTreeMap<u32, Vec<char>> = BTreeMap::new();

	let (inp_stacks, instructions) = input.split_once("\n\n").unwrap();
	let width = inp_stacks.split_once('\n').unwrap().0.len();

	for i in (1..=(((width + 1) / 4).pow(2))).step_by(4) {
		let col = inp_stacks
			.lines()
			.map(|line| line.chars().nth(i).unwrap_or(' '))
			.filter(|x| !x.is_whitespace())
			.collect::<Vec<_>>();
		if !col.is_empty() {
			let (num, col) = col.split_last().unwrap();
			let num = num.to_digit(10).unwrap();
			let mut col = col.to_vec();
			col.reverse();
			stacks.insert(num, col);
		}
	}

	let instructions: Vec<_> = instructions
		.lines()
		.map(|line| {
			let mut parts = line
				.split_whitespace()
				.filter_map(|x| x.parse::<i32>().ok());
			(
				parts.next().unwrap(),
				parts.next().unwrap(),
				parts.next().unwrap(),
			)
		})
		.collect();

	for (amount, from, to) in instructions {
		if let Some(vec) = stacks.get_mut(&(from as u32)) {
			let mut retrived_items = vec.drain(vec.len() - amount as usize..).collect();
			if let Some(vec) = stacks.get_mut(&(to as u32)) {
				vec.append(&mut retrived_items);
			}
		}
	}

	let answer = stacks
		.iter()
		.map(|(_, val)| val.last().unwrap())
		.collect::<String>();

	answer
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
	// #[ignore]
	fn part2() {
		let result = process_part2(TEST_INPUT);
		assert_eq!(result, "MCD");
	}
}
