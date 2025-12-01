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
    let mut position = INITIAL_POSITION;
    let mut zero_count = 0;

    for instruction in parse::<Instruction>(input)? {
        let motion = match instruction.direction {
            Direction::Left => -instruction.qty,
            Direction::Right => instruction.qty,
        };
        let next_position = position + motion;

        let position_bounded = position.rem_euclid(DIAL_SIZE);
        let next_position_bounded = next_position.rem_euclid(DIAL_SIZE);

        let mut passed_zeros = {
            // "day" is a bad term here, but I'm blanking on anything better.
            // it's an identifier for a particular 0-99 range of the dial;
            // a full spin of the wheel.
            // if we're in the same day, we have not clicked past 0.
            // If we're in a different day, we have clicked past 0 some amount of times.
            let start_dial_day = position.div_euclid(DIAL_SIZE);
            let end_dial_day = next_position.div_euclid(DIAL_SIZE);
            start_dial_day.abs_diff(end_dial_day)
        };
        // a limitaiton of the day count mechanism: left turns from 0 produce an extra false count
        if position_bounded == 0 && instruction.direction == Direction::Left && passed_zeros > 0 {
            passed_zeros -= 1;
        }
        // a second limitation of the day count mechanism: left turns onto 0 undercount
        if next_position_bounded == 0 && instruction.direction == Direction::Left {
            passed_zeros += 1;
        }

        zero_count += passed_zeros;
        position = next_position;
    }

    println!("zero count (pt 2): {zero_count}");
    Ok(())
}
