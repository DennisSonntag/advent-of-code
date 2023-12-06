pub fn process_part1(input: &str) -> u32 {
    let chars input.lines().enumerate().map(|(y, line)| line.chars().enumerate());


    14
}

pub fn process_part2(input: &str) -> u32 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn part1_works() {
        let result = process_part1(TEST_INPUT);
        assert_eq!(result, 4361);
    }

    #[test]
    #[ignore]
    fn part2_works() {
        let result = process_part2(TEST_INPUT);
        assert_eq!(result, 2286);
    }
}
