use aoclib::parse;
use color_eyre::{
    eyre::{format_err, Report},
    Result,
};
use std::{path::Path, str::FromStr};

const DIAL_SIZE: i32 = 100;
const INITIAL_POSITION: i32 = 50;

#[derive(Debug, Clone, Copy, PartialEq, Eq, parse_display::Display, parse_display::FromStr)]
enum Direction {
    #[display("L")]
    Left,
    #[display("R")]
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, parse_display::Display)]
#[display("{direction}{qty}")]
struct Instruction {
    direction: Direction,
    qty: i32,
}

impl FromStr for Instruction {
    type Err = Report;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if !s.is_char_boundary(1) {
            return Err(format_err!("index 1 not a unicode boundary in {}", s));
        }
        let (direction, qty) = s.split_at(1);
        let direction = direction.parse()?;
        let qty = qty.parse()?;
        Ok(Self { direction, qty })
    }
}

pub fn part1(input: &Path) -> Result<()> {
    let mut position = INITIAL_POSITION;
    let mut zero_count = 0;
    for instruction in parse::<Instruction>(input)? {
        let motion = match instruction.direction {
            Direction::Left => -instruction.qty,
            Direction::Right => instruction.qty,
        };
        position += motion;

        if position.rem_euclid(DIAL_SIZE) == 0 {
            zero_count += 1;
        }
    }
    println!("zero count: {zero_count}");
    Ok(())
}

pub fn part2(input: &Path) -> Result<()> {
    unimplemented!("input file: {:?}", input)
}
