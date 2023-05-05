use std::{cmp::Ordering, str::FromStr};

#[derive(PartialEq, Clone, Copy)]
pub enum Move {
	Rock = 1,
	Paper = 2,
	Sciscors = 3,
}

impl PartialOrd for Move {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		if self == &Self::Sciscors && other == &Self::Rock {
			Some(Ordering::Less)
		} else if self == &Self::Rock && other == &Self::Sciscors {
			Some(Ordering::Greater)
		} else {
			Some((*self as u8).cmp(&(*other as u8)))
		}
	}
}

impl FromStr for Move {
	type Err = String;

	fn from_str(s: &str) -> Result<Move, Self::Err> {
		match s {
			"A" | "X" => Ok(Move::Rock),
			"B" | "Y" => Ok(Move::Paper),
			"C" | "Z" => Ok(Move::Sciscors),
			_ => Err(String::from("not a valid move string")),
		}
	}
}

pub fn process_part1(input: &str) -> u32 {
	// 0 lost
	// 3 draw
	// 6 won

	let result: u32 = input
		.lines()
		.map(|line| {
			let moves: Vec<Move> = line
				.split(" ")
				.map(|s| s.parse::<Move>().unwrap())
				.collect();
			match moves[0].partial_cmp(&moves[1]) {
				Some(Ordering::Equal) => 3 + moves[1] as u32,
				Some(Ordering::Less) => 6 + moves[1] as u32,
				Some(Ordering::Greater) => 0 + moves[1] as u32,
				None => panic!("moves should be comparable"),
			}
		})
		.sum();
	result
}

pub fn process_part2(input: &str) -> u32 {
	// y : draw
	// x : lose
	// z : win

	#[derive(PartialEq, Clone, Copy)]
	pub enum Outcome {
		Lose = 0,
		Draw = 3,
		Win = 6,
	}

	impl FromStr for Outcome {
		type Err = String;

		fn from_str(s: &str) -> Result<Outcome, Self::Err> {
			match s {
				"X" => Ok(Outcome::Lose),
				"Y" => Ok(Outcome::Draw),
				"Z" => Ok(Outcome::Win),
				_ => Err(String::from("not a valid move string")),
			}
		}
	}

	let result: u32 = input
		.lines()
		.map(|line| {
			let data = line.split(' ').collect::<Vec<_>>();
			let moves = data[0].parse::<Move>().unwrap();
			let outcome = data[1].parse::<Outcome>().unwrap();
			// 0 lost
			// 3 draw
			// 6 won

			// 1 Rock
			// 2 Paper
			// 3 Sciscors

			match moves {
				Move::Rock => {
					match outcome {
						//Sciscors
						Outcome::Lose => 0 + 3,
						//Rock
						Outcome::Draw => 3 + 1,
						//Paper
						Outcome::Win => 6 + 2,
					}
				}
				Move::Paper => {
					match outcome {
						//Rock
						Outcome::Lose => 0 + 1,
						//Paper
						Outcome::Draw => 3 + 2,
						//Sciscors
						Outcome::Win => 6 + 3,
					}
				}
				Move::Sciscors => {
					match outcome {
						//Paper
						Outcome::Lose => 0 + 2,
						//Sciscors
						Outcome::Draw => 3 + 3,
						//Rock
						Outcome::Win => 6 + 1,
					}
				}
			}
		})
		.sum();
	result
}

#[cfg(test)]
mod tests {
	use super::*;

	const TEST_INPUT: &str = "A Y
B X
C Z";

	#[test]
	fn part1() {
		let result = process_part1(TEST_INPUT);
		assert_eq!(result, 15);
	}

	#[test]
	fn part2() {
		let result = process_part2(TEST_INPUT);
		assert_eq!(result, 12);
	}
}
