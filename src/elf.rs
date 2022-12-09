use std::str::FromStr;
use nom::character::complete::{digit1, line_ending};
use nom::combinator::{map, map_res};
use nom::IResult;
use nom::multi::{count, separated_list0};

#[derive(Debug)]
pub struct Elf {
    inventory: Vec<i32>,
}

impl Elf {
    pub fn inventory_total(elf: &Elf) -> i32 {
        elf.inventory.iter().sum::<i32>()
    }

    pub fn parse(input: &str) -> IResult<&str, Self> {
        let parse_numbers = separated_list0(line_ending, map_res(digit1, i32::from_str));

        map(parse_numbers, |inventory| Elf { inventory })(input)
    }

    pub fn parse_many(input: &str) -> IResult<&str, Vec<Self>> {
        separated_list0(count(line_ending, 2), Self::parse)(input)
    }
}
