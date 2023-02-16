pub fn process_part1(input: &str) -> u32 {
	let result = input
		.split("\n\n")
		.map(|elm| {
			elm.lines()
				.map(|num| num.parse::<u32>().unwrap())
				.into_iter()
				.sum::<u32>()
		})
		.max()
		.unwrap();
	result
}

pub fn process_part2(input: &str) -> u32 {
	let mut result = input
		.split("\n\n")
		.map(|elm| {
			elm.lines()
				.map(|num| num.parse::<u32>().unwrap())
				.into_iter()
				.sum::<u32>()
		})
		.collect::<Vec<_>>();
	result.sort_by(|a, b| b.cmp(a));
	result.iter().take(3).sum()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn part1() {
		let test_input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
		let result = process_part1(test_input);
		assert_eq!(result, 24000);
	}

	#[test]
	fn part2() {
		let test_input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
		let result = process_part2(test_input);
		assert_eq!(result, 45000);
	}
}
