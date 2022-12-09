use crate::elf::Elf;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<Elf> {
    Elf::parse_many(input).unwrap().1
}

#[aoc(day1, part1)]
pub fn solve_part1(elves: &[Elf]) -> i32 {
    elves
        .iter()
        .map(Elf::inventory_total)
        .fold(i32::MIN, |a: i32,b| a.max(b))
}

#[aoc(day1, part2)]
pub fn solve_part2(elves: &[Elf]) -> i32 {

    let mut totals: Vec<i32> = elves
        .iter()
        .map(Elf::inventory_total)
        .collect();

    totals.sort_by(|a,b| b.cmp(a));

    totals
        .iter()
        .take(3)
        .sum()
}