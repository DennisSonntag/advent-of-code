#![allow(dead_code)]

pub fn process_part1(input: &str) -> usize {
	let trees = input
		.lines()
		.map(|line| {
			line.chars()
				.map(|x| x.to_digit(10).unwrap() as u8)
				.collect::<Vec<u8>>()
		})
		.collect::<Vec<_>>();

	let mut tree_visibility = trees
		.iter()
		.map(|x| x.iter().map(|_| false).collect::<Vec<_>>())
		.collect::<Vec<_>>();

    tree_visibility = tree_visibility.iter().enumerate().map(|(idx, x)| {

    })

	// // left to right
	// for x in 0..trees.len() {
	// 	let mut max_height = 0;
	// 	for y in 0..trees[0].len() {
	// 		if trees[x][y] > max_height {
	// 			max_height = trees[x][y];
	// 			tree_visibility[x][y] = true;
	// 		}else {
	// 			tree_visibility[x][y] = false;
	//
	// 		}
	// 	}
	// }
	//
	// // left to right
	// for x in (0..trees.len()).rev() {
	// 	let mut max_height = 0;
	// 	for y in 0..trees[0].len() {
	// 		if trees[x][y] > max_height {
	// 			max_height = trees[x][y];
	// 			tree_visibility[x][y] = true;
	// 		}else {
	// 			tree_visibility[x][y] = false;
	//
	// 		}
	// 	}
	// }

    dbg!(&trees);
    dbg!(&tree_visibility);

    tree_visibility.iter().flatten().filter(|&&v| v).count()
}

pub fn process_part2(input: &str) -> u32 {
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
