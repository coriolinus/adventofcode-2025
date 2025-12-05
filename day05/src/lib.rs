use color_eyre::{
    Result,
    eyre::{Context, OptionExt, eyre},
};
use std::{
    io::{BufRead, BufReader},
    path::Path,
    str::FromStr,
};

type IngredientId = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Range {
    low: IngredientId,
    high: IngredientId,
}

impl Range {
    fn contains(&self, ingredient: IngredientId) -> bool {
        self.low <= ingredient && self.high >= ingredient
    }
}

impl FromStr for Range {
    type Err = color_eyre::eyre::Report;

    fn from_str(s: &str) -> Result<Self> {
        let dash = s.find('-').ok_or_eyre("no hyphen")?;
        let (low, rest) = s.split_at(dash);
        let (_dash, high) = rest.split_at(1);
        let low = low.parse()?;
        let high = high.parse()?;
        Ok(Self { low, high })
    }
}

#[derive(Debug)]
pub struct Input {
    fresh_ranges: Vec<Range>,
    available: Vec<IngredientId>,
}

impl Input {
    fn from_path(input: &Path) -> Result<Self> {
        let mut fresh_ranges = Vec::new();
        let mut available = Vec::new();

        let reader = BufReader::new(std::fs::File::open(input).wrap_err("opening input file")?);
        for (line_number_0, line) in reader.lines().enumerate() {
            let line_number = line_number_0 + 1;
            let line = line.wrap_err_with(|| format!("reading line {line_number}"))?;

            if line.is_empty() {
                continue;
            }
            if let Ok(range) = line.parse::<Range>() {
                fresh_ranges.push(range);
            } else if let Ok(ingredient) = line.parse::<IngredientId>() {
                available.push(ingredient);
            } else {
                return Err(eyre!("failed to parse line {line_number}: {line}"));
            }
        }

        Ok(Self {
            fresh_ranges,
            available,
        })
    }

    fn consolidate_ranges(&mut self) {
        self.fresh_ranges.sort_unstable_by_key(|range| range.low);
        let mut consolidated = Vec::new();

        // index of range containing low bound
        let mut low_idx = 0;
        while low_idx < self.fresh_ranges.len() {
            let low = self.fresh_ranges[low_idx].low;
            let mut high = self.fresh_ranges[low_idx].high;
            let mut high_idx = low_idx + 1;

            while let Some(high_range) = self.fresh_ranges.get(high_idx).copied()
                && high_range.low <= high + 1
            {
                high_idx += 1;
                high = high.max(high_range.high);
            }

            consolidated.push(Range { low, high });
            low_idx = high_idx;
        }

        self.fresh_ranges = consolidated;
    }

    fn is_fresh(&self, ingredient: IngredientId) -> bool {
        debug_assert!(
            self.fresh_ranges
                .windows(2)
                .all(|window| window[0].high < window[1].low),
            "all ranges must be consolidated before calling this method"
        );
        // could maybe get fancy with a binary search, but let's try this for now
        self.fresh_ranges
            .iter()
            .any(|range| range.contains(ingredient))
    }
}

pub fn part1(input: &Path) -> Result<()> {
    let mut input = Input::from_path(input)?;
    input.consolidate_ranges();
    let n_fresh = input
        .available
        .iter()
        .copied()
        .filter(|&ingredient| input.is_fresh(ingredient))
        .count();
    println!("n fresh ingredients: {n_fresh}");
    Ok(())
}

pub fn part2(input: &Path) -> Result<()> {
    unimplemented!("input file: {:?}", input)
}
