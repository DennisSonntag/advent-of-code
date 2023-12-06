#[derive(Debug, Clone)]
struct Cube<'a> {
    color: &'a str,
    count: u32,
}

#[derive(Debug, Clone, Default)]
struct Game<'a> {
    id: u32,
    rounds: Vec<Cube<'a>>,
}

#[derive(Debug, Clone, Default)]
struct Colors {
    red: Vec<u32>,
    green: Vec<u32>,
    blue: Vec<u32>,
}

fn process_line(line: &str) -> Game {
    let (id, rounds) = line.split_once(": ").unwrap();

    let id = id.split_once(' ').unwrap().1.parse::<u32>().unwrap();

    let rounds = rounds
        .split("; ")
        .flat_map(|set| {
            set.split(", ").map(|x| {
                let (left, color) = x.split_once(' ').unwrap();
                let count = left.parse::<u32>().unwrap();

                Cube { color, count }
            })
        })
        .collect();
    Game { id, rounds }
}

pub fn process_part1(input: &str) -> u32 {
    input
        .lines()
        .map(process_line)
        .filter(|game| {
            game.rounds.iter().all(|set| match set.color {
                "red" => set.count <= 12,
                "green" => set.count <= 13,
                "blue" => set.count <= 14,
                _ => false,
            })
        })
        .map(|game| game.id)
        .sum()
}

pub fn process_part2(input: &str) -> u32 {
    input
        .lines()
        .map(process_line)
        .map(|game| {
            let colors = game.rounds.iter().fold(
                Colors {
                    red: Vec::new(),
                    green: Vec::new(),
                    blue: Vec::new(),
                },
                |mut acc, x| {
                    match x.color {
                        "red" => acc.red.push(x.count),
                        "blue" => acc.blue.push(x.count),
                        "green" => acc.green.push(x.count),
                        _ => {}
                    };
                    acc
                },
            );

            return colors.red.iter().max().unwrap()
                * colors.green.iter().max().unwrap()
                * colors.blue.iter().max().unwrap();
        })
        .sum()
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
    #[ignore]
    fn part1_works() {
        let result = process_part1(TEST_INPUT);
        assert_eq!(result, 8);
    }

    #[test]
    fn part2_works() {
        let result = process_part2(TEST_INPUT);
        assert_eq!(result, 2286);
    }
}
