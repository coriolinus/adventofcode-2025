use aoclib::{parse, CommaSep};
use color_eyre::{
    eyre::{Context, OptionExt, Report},
    Result,
};
use std::{path::Path, str::FromStr};

type ProductId = u64;

struct ProductIdRange {
    first_id: ProductId,
    last_id: ProductId,
}

impl FromStr for ProductIdRange {
    type Err = Report;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let hyphen_position = s.find('-').ok_or_eyre("no hyphen")?;
        let (first_id, s) = s.split_at(hyphen_position);
        let (_, last_id) = s.split_at(1);
        let first_id = first_id.parse().wrap_err("parsing first id")?;
        let last_id = last_id.parse().wrap_err("parsing last id")?;
        Ok(Self { first_id, last_id })
    }
}

impl IntoIterator for ProductIdRange {
    type Item = ProductId;

    type IntoIter = std::ops::RangeInclusive<ProductId>;

    fn into_iter(self) -> Self::IntoIter {
        let Self { first_id, last_id } = self;
        first_id..=last_id
    }
}

fn id_is_valid(id: ProductId) -> bool {
    let s = id.to_string();
    let half_len = s.len() / 2;
    if !s.is_char_boundary(half_len) {
        eprintln!("WARNING: half_len was not on char boundary");
        return false;
    }
    let (first_half, second_half) = s.split_at(half_len);
    first_half.len() != second_half.len() || first_half != second_half
}

pub fn part1(input: &Path) -> Result<()> {
    for (n, row) in parse::<CommaSep<ProductIdRange>>(input)?.enumerate() {
        println!("part 1 row {n}:");

        let sum_invalid_ids = row
            .into_iter()
            .flat_map(|id_range| id_range.into_iter())
            .filter(|id| !id_is_valid(*id))
            .sum::<u64>();
        println!(" sum of invalid ids: {sum_invalid_ids}");
    }

    Ok(())
}

pub fn part2(input: &Path) -> Result<()> {
    unimplemented!("input file: {:?}", input)
}
