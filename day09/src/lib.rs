use aoclib::parse;
use color_eyre::{eyre::OptionExt, Result};
use itertools::Itertools;
use std::path::Path;

#[derive(Debug, Clone, Copy, parse_display::FromStr, parse_display::Display)]
#[display("{x},{y}")]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn area(&self, other: &Self) -> u64 {
        (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
    }
}

pub fn part1(input: &Path) -> Result<()> {
    let points = parse::<Point>(input)?.collect::<Vec<_>>();
    let max_area = points
        .iter()
        .cartesian_product(points.iter())
        .map(|(left, right)| left.area(right))
        .max()
        .ok_or_eyre("no points to consider")?;

    println!("largest area: {max_area}");
    Ok(())
}

pub fn part2(input: &Path) -> Result<()> {
    unimplemented!("input file: {:?}", input)
}
