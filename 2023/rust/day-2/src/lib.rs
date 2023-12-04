#![allow(unused, dead_code, clippy::panic)]

enum Color {
    Red,
    Green,
    Blue,
}

struct Count {
    color: Color,
    amount: u32,
}

pub fn process_part1(input: &str) -> u32 {
    let thing = input
        .lines()
        .map(|line| {
            let id = line
                .split_once(':')
                .unwrap()
                .0
                .split_once(' ')
                .unwrap()
                .1
                .parse::<u32>()
                .unwrap();
            let sets = line
                .split_once(':')
                .unwrap()
                .1
                .split(';')
                .map(|set| {
                    set.split(',')
                        .map(|x| {
                            let (count, right) = x.split_once(' ').unwrap();
                            let count = count.parse::<u32>().unwrap();
                            let color = match right {
                                "red" => Color::Red,
                                "green" => Color::Green,
                                "blue" => Color::Blue,
                            };

                            return Count {
                                color,
                                amount: count,
                            };
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            sets
        })
        .collect::<Vec<_>>();
    println!("{thing:?}");

    14
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
