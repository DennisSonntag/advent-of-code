#![allow(dead_code)]

pub fn process_part1(input: &str) -> usize {
	let trees: Vec<Vec<u8>> = input
		.lines()
		.map(|line| {
			line.chars()
				.map(|x| x.to_digit(10).unwrap() as u8)
				.collect()
		})
		.collect();

	let max_length = trees.len();
	let line_max_length = trees[0].len();

	let mut visable_trees: Vec<Vec<bool>> = trees
		.iter()
		.enumerate()
		.map(|(i, tree_line)| {
			tree_line
				.iter()
				.enumerate()
				.map(|(line_idx, _)| {
					i == 0
						|| i == max_length - 1 || line_idx == 0
						|| line_idx == line_max_length - 1
				})
				.collect()
		})
		.collect();

	for row in 0..max_length {
		let mut current_tree_height = 0;
		let mut current_tree_height1 = 0;
		let mut current_tree_height2 = 0;
		let mut current_tree_height3 = 0;
		for col in 0..line_max_length {
			// left to right
			// if col == 0 {
			// 	current_tree_height = trees[row][col];
			// } else if trees[row][col] > current_tree_height {
			// 	current_tree_height = trees[row][col];
			// 	visable_trees[row][col] = true;
			// }

			// down to up
			// if col == max_length - 1 {
			// 	current_tree_height1 = trees[col][row];
			// } else if trees[col][row] > current_tree_height1 {
			// 	current_tree_height1 = trees[col][row];
			// 	visable_trees[col][row] = true;
			// }

			let row = max_length - 1 - row;
			let col = line_max_length - 1 - col;
			// right to left
			if row == max_length - 1 {
				current_tree_height2 = trees[row][col];
			} else if trees[row][col] > current_tree_height2 {
				current_tree_height2 = trees[row][col];
				visable_trees[row][col] = true;
			}

			// up to down
			if col == max_length - 1 {
				current_tree_height3 = trees[col][row];
			} else if trees[col][row] > current_tree_height3 {
				current_tree_height3 = trees[col][row];
				visable_trees[col][row] = true;
			}
		}
	}

	visable_trees.iter().flatten().filter(|&&v| v).count()
}

pub fn process_part2(_input: &str) -> u32 {
	unimplemented!()
}

#[cfg(test)]
mod tests {

	use super::*;
	const TEST_INPUT: &str = "30373
25512
65332
33549
35390";

	#[test]
	// #[ignore]
	fn part1() {
		let result = process_part1(TEST_INPUT);
		assert_eq!(result, 21);
	}

	#[test]
	#[ignore]
	fn part2() {
		let result = process_part2(TEST_INPUT);
		assert_eq!(result, 24933642);
	}
}
