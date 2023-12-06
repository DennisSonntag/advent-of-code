#![allow(unused, dead_code, clippy::panic)]

use std::ops::Add;

#[derive(Debug, Clone)]
struct Count {
    color: String,
    count: u32,
}

#[derive(Debug, Clone, Default)]
struct Game {
    id: u32,
    sets: Vec<Count>,
}

fn process_line(line: &str) -> Game {
    let (id, games) = line.split_once(": ").unwrap();

    let id = id.split_once(' ').unwrap().1.parse::<u32>().unwrap();

    let games = games
        .split("; ")
        .flat_map(|set| {
            set.split(", ").map(|x| {
                let (left, right) = x.split_once(' ').unwrap();
                let count = left.parse::<u32>().unwrap();
                let color = right.to_string();

                Count { color, count }
            })
        })
        .collect::<Vec<_>>();

    // .fold(
    //     Game {
    //         id,
    //         ..Default::default()
    //     },
    //     |mut acc, x| {
    //         match x.color.as_str() {
    //             "red" => acc.red += x.count,
    //             "green" => acc.green += x.count,
    //             "blue" => acc.blue += x.count,
    //             _ => {}
    //         };
    //         acc
    //     },
    // );
    Game { id, sets: games }
}

pub fn process_part1(input: &str) -> u32 {
    input
        .lines()
        .map(process_line)
        .filter(|game| {
            game.sets.iter().all(|set| {
                if set.color == "red" {
                    return set.count <= 12;
                } else if set.color == "green" {
                    return set.count <= 13;
                } else {
                    return set.count <= 14;
                }
            })
        })
        .map(|game| game.id)
        .sum()
}

pub fn process_part2(input: &str) -> u32 {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn part1_works() {
        let result = process_part1(TEST_INPUT);
        assert_eq!(result, 8);
    }

    #[test]
    #[ignore]
    fn part2_works() {
        let result = process_part2(TEST_INPUT);
        assert_eq!(result, 281);
    }
}
