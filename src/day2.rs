use crate::rock_paper_scissors::{Round, FixedRound};

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> String {
    String::from(input)
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let rounds = Round::parse_many(input).unwrap().1;

    rounds
        .iter()
        .map(Round::score)
        .sum()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &str) -> i32 {
    let fixed_rounds = FixedRound::parse_many(input).unwrap().1;

    fixed_rounds
        .iter()
        .map(FixedRound::score)
        .sum()
}
