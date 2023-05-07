pub fn process_part1(input: &str) -> usize {
	let pairs = input
		.lines()
		.map(|x| {
			let (left, right) = x.split_once(',').unwrap();

			let left = left
				.split('-')
				.map(|y| y.parse::<u32>().unwrap())
				.collect::<Vec<_>>();
			let left = (left[0])..=(left[1]);

			let right = right
				.split('-')
				.map(|y| y.parse::<u32>().unwrap())
				.collect::<Vec<_>>();
			let right = (right[0])..=(right[1]);

			(left, right)
		})
		.collect::<Vec<_>>();

	let answer = pairs
		.iter()
		.filter(|(left, right)| {
			let left_has_a = left.clone().all(|x| right.contains(&x));
			let right_has_a = right.clone().all(|x| left.contains(&x));

			left_has_a || right_has_a
		})
		.count();

	answer
}

pub fn process_part2(input: &str) -> usize {
	let pairs = input
		.lines()
		.map(|x| {
			let (left, right) = x.split_once(',').unwrap();

			let left = left
				.split('-')
				.map(|y| y.parse::<u32>().unwrap())
				.collect::<Vec<_>>();
			let left = (left[0])..=(left[1]);

			let right = right
				.split('-')
				.map(|y| y.parse::<u32>().unwrap())
				.collect::<Vec<_>>();
			let right = (right[0])..=(right[1]);

			(left, right)
		})
		.collect::<Vec<_>>();

	let answer = pairs
		.iter()
		.filter(|(left, right)| {
			let left_has_a = left.clone().any(|x| right.contains(&x));
			let right_has_a = right.clone().any(|x| left.contains(&x));

			left_has_a || right_has_a
		})
		.count();

	answer
}

#[cfg(test)]
mod tests {
	use super::*;

	const TEST_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

	#[test]
	fn part1() {
		let result = process_part1(TEST_INPUT);
		assert_eq!(result, 2);
	}

	#[test]
	#[ignore]
	fn part2() {
		let result = process_part2(TEST_INPUT);
		assert_eq!(result, 4);
	}
}
