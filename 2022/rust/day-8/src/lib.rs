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

	let get_neighbors = |row: usize, col: usize| -> Vec<u8> {
		let mut left = None; let mut bottom = None; if col > 0 { left = Some(trees[row][col - 1]);
		}
		if row > 0 {
			bottom = Some(trees[row - 1][col]);
		}
		let right = Some(trees[row][col + 1]);
		let top = Some(trees[row + 1][col]);

		[top, right, bottom, left]
			.iter()
			.filter(|x| x.is_some())
			.map(|x| x.unwrap())
			.collect()
	};

	let mut visable_trees = visable_trees
		.iter()
		.flatten()
		.map(|x| x.to_owned())
		.collect::<Vec<_>>();
	let trees = trees
		.iter()
		.flatten()
		.map(|x| x.to_owned())
		.collect::<Vec<_>>();

	let mut tallest_left_right = 0;
	let mut tallest_right_left = 0;
	let mut tallest_up_down = 0;
	let mut tallest_down_up = 0;
	for (i, val) in trees.iter().enumerate() {
		let row = i % line_max_length;
		let col = i % max_length;
		let idx = row * max_length + col;

		// left to right
		if col == 0 {
			tallest_left_right = trees[idx];
		} else if trees[idx] > tallest_left_right {
			tallest_left_right = trees[idx];
			visable_trees[idx] = true;
		}

		// down to up
		if col == max_length - 1 {
			tallest_right_left = trees[idx];
		} else if trees[idx] > tallest_right_left {
			tallest_right_left = trees[idx];
			visable_trees[idx] = true;
		}

		let row = max_length - 1 - row;
		let col = line_max_length - 1 - col;
		let idx = row * max_length + col;
		// right to left
		if row == max_length - 1 {
			tallest_up_down = trees[idx];
		} else if trees[idx] > tallest_up_down {
			tallest_up_down = trees[idx];
			visable_trees[idx] = true;
		}

		// up to down
		if col == max_length - 1 {
			tallest_down_up = trees[idx];
		} else if trees[idx] > tallest_down_up {
			tallest_down_up = trees[idx];
			visable_trees[idx] = true;
		}
	}

	// for row in 0..max_length {
	// 	let mut current_tree_height = 0;
	// 	let mut current_tree_height1 = 0;
	// 	let mut current_tree_height2 = 0;
	// 	let mut current_tree_height3 = 0;
	// 	for col in 0..line_max_length {
	//
	// 		// left to right
	// 		if col == 0 {
	// 			current_tree_height = trees[row][col];
	// 		} else if trees[row][col] > current_tree_height {
	// 			current_tree_height = trees[row][col];
	// 			visable_trees[row][col] = true;
	// 		}
	//
	// 		// down to up
	// 		if col == max_length - 1 {
	// 			current_tree_height1 = trees[col][row];
	// 		} else if trees[col][row] > current_tree_height1 {
	// 			current_tree_height1 = trees[col][row];
	// 			visable_trees[col][row] = true;
	// 		}
	//
	// 		let row = max_length - 1 - row;
	// 		let col = line_max_length - 1 - col;
	// 		// right to left
	// 		if row == max_length - 1 {
	// 			current_tree_height2 = trees[row][col];
	// 		} else if trees[row][col] > current_tree_height2 {
	// 			current_tree_height2 = trees[row][col];
	// 			visable_trees[row][col] = true;
	// 		}
	//
	// 		// up to down
	// 		if col == max_length - 1 {
	// 			current_tree_height3 = trees[col][row];
	// 		} else if trees[col][row] > current_tree_height3 {
	// 			current_tree_height3 = trees[col][row];
	// 			visable_trees[col][row] = true;
	// 		}
	// 	}
	// }

	// visable_trees.iter().flatten().filter(|&&v| v).count()
	visable_trees.iter().filter(|&&v| v).count()
	// 123123
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
