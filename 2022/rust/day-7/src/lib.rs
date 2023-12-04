#![allow(dead_code)]

use std::collections::BTreeMap;

#[derive(Debug, Clone)]
enum Operation {
	Cd(Cd),
	Ls(Vec<Files>),
}

#[derive(Debug, Clone)]
enum Cd {
	Root,
	Up,
	Down(String),
}
#[derive(Debug, Clone)]
enum Files {
	File { size: u32, name: String },
	Dir(String),
}

fn parse_input(input: &str) -> Vec<Operation> {
	let mut commands: Vec<Operation> = vec![];
	for line in input.lines() {
		let tokens = line.split_whitespace().collect::<Vec<_>>();
		match (tokens[0], tokens.get(1), tokens.get(2)) {
			("$", Some(&"cd"), Some(&"..")) => commands.push(Operation::Cd(Cd::Up)),
			("$", Some(&"cd"), Some(&"/")) => commands.push(Operation::Cd(Cd::Root)),
			("$", Some(&"cd"), Some(name)) => {
				commands.push(Operation::Cd(Cd::Down(name.to_string())))
			}
			("$", Some(&"ls"), None) => {
				commands.push(Operation::Ls(vec![]));
				continue;
			}
			("dir", Some(name), None) => {
				if let Operation::Ls(files) = commands.last_mut().unwrap() {
					files.push(Files::Dir(name.to_string()));
					continue;
				}
			}
			(size, Some(name), None) => {
				if let Operation::Ls(files) = commands.last_mut().unwrap() {
					files.push(Files::File {
						size: size.parse().unwrap(),
						name: name.to_string(),
					});
					continue;
				}
			}
			_ => (),
		}
	}
	commands
}
fn calculate_sizes<'a>(
	(mut context, mut sizes): (Vec<&'a str>, BTreeMap<Vec<&'a str>, u32>),
	command: &'a Operation,
) -> (Vec<&'a str>, BTreeMap<Vec<&'a str>, u32>) {
	match command {
		Operation::Cd(Cd::Root) => {
			context.push("");
		}
		Operation::Cd(Cd::Up) => {
			context.pop();
		}
		Operation::Cd(Cd::Down(name)) => {
			context.push(name);
		}
		Operation::Ls(files) => {
			let sum = files
				.iter()
				.filter_map(|file| {
					if let Files::File { size, .. } = file {
						Some(size)
					} else {
						None
					}
				})
				.sum::<u32>();
			for i in 0..context.len() {
				sizes
					.entry(context[0..=i].to_vec())
					.and_modify(|v| *v += sum)
					.or_insert(sum);
			}
		}
	};
	(context, sizes)
}

pub fn process_part1(input: &str) -> u32 {
	let commands = parse_input(input);

	let (_, sizes) = commands
		.iter()
		.fold((Vec::new(), BTreeMap::new()), calculate_sizes);

	sizes
		.iter()
		.filter(|(_, &size)| size < 100_000)
		.map(|(_, size)| size)
		.sum::<u32>()
}

pub fn process_part2(input: &str) -> u32 {
	let commands = parse_input(input);

	let (_, sizes) = commands
		.iter()
		.fold((Vec::new(), BTreeMap::new()), calculate_sizes);

	let total_size = 70_000_000;
	let target_unused = 30_000_000;

	let used_space = sizes.get(&Vec::from([""])).unwrap();
	let current_free_space = total_size - used_space;
	let need_to_free_at_least = target_unused - current_free_space;

	let mut valid_dirs = sizes
		.iter()
		.filter(|(_, &size)| size > need_to_free_at_least)
		.map(|(_, size)| size)
		.collect::<Vec<&u32>>();

	valid_dirs.sort();
	**valid_dirs.first().unwrap()
}

#[cfg(test)]
mod tests {

	use super::*;
	const TEST_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

	#[test]
	#[ignore]
	fn part1() {
		let result = process_part1(TEST_INPUT);
		assert_eq!(result, 95437);
	}

	#[test]
	// #[ignore]
	fn part2() {
		let result = process_part2(TEST_INPUT);
		assert_eq!(result, 24933642);
	}
}
