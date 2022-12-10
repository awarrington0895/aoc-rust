use nom::character::complete::{alpha1, line_ending, space1};
use nom::combinator::{map, map_res};
use nom::multi::separated_list0;
use nom::sequence::separated_pair;
use nom::IResult;
use std::str::FromStr;

type Score = i32;

#[derive(Debug, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    pub fn value(&self) -> Score {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    pub fn parse(input: &str) -> IResult<&str, Self> {
        map_res(alpha1, Shape::from_str)(input)
    }
}

enum Outcome {
    Victory,
    Defeat,
    Draw,
}

impl Outcome {
    pub fn score(&self) -> Score {
        match self {
            Outcome::Victory => 6,
            Outcome::Draw => 3,
            Outcome::Defeat => 0,
        }
    }
}

#[derive(Debug)]
pub struct Round {
    player: Shape,
    opponent: Shape,
}

impl Round {
    pub fn score(round: &Round) -> Score {
        round.player.value() + round.determine_outcome().score()
    }

    pub fn parse_many(input: &str) -> IResult<&str, Vec<Self>> {
        let parse_single = map(
            separated_pair(Shape::parse, space1, Shape::parse),
            |(opponent, player)| Self { player, opponent },
        );

        separated_list0(line_ending, parse_single)(input)
    }

    fn determine_outcome(&self) -> Outcome {
        match (&self.player, &self.opponent) {
            (Shape::Rock, Shape::Scissors)
            | (Shape::Paper, Shape::Rock)
            | (Shape::Scissors, Shape::Paper) => Outcome::Victory,
            _ if self.player == self.opponent => Outcome::Draw,
            _ => Outcome::Defeat,
        }
    }
}

impl FromStr for Shape {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Shape::Rock),
            "B" | "Y" => Ok(Shape::Paper),
            "C" | "Z" => Ok(Shape::Scissors),
            _ => Err(()),
        }
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Round> {
    Round::parse_many(input).unwrap().1
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Round]) -> i32 {
    input.iter().map(Round::score).sum()
}
