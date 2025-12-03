use aoclib::parse;
use color_eyre::{
    eyre::{eyre, OptionExt, Report},
    Result,
};
use std::{path::Path, str::FromStr};

/// Battery bank
struct Bank(Vec<u8>);

impl FromStr for Bank {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self> {
        let batteries = s
            .chars()
            .map(|char| {
                char.to_digit(10)
                    .map(|value| value as u8)
                    .ok_or_eyre("non-numeric byte in input")
            })
            .collect::<Result<_, _>>()?;
        Ok(Self(batteries))
    }
}

impl Bank {
    fn joltage_from_indices(&self, left: usize, right: usize) -> u32 {
        debug_assert!(left < right, "invalid index choice");
        u32::from(self.0[left] * 10) + u32::from(self.0[right])
    }

    fn select_indices_pt1(&self) -> Result<(usize, usize)> {
        if self.0.len() < 2 {
            return Err(eyre!("bank has too few batteries"));
        }

        let (left, _) = self
            .0
            .iter()
            .enumerate()
            // we rev here because max_by_key takes the last of any equally maximum elements,
            // and for our algorithm's correctness we have to have the leftmost maximum value
            //
            // also note that we rev _after_ we enumerate, so that the indices correctly count down as we go
            .rev()
            .skip(1)
            .max_by_key(|(_idx, value)| **value)
            .expect("maxing a non-empty list always produces something");

        let (right, _) = self
            .0
            .iter()
            .enumerate()
            .skip(left + 1)
            .max_by_key(|(_idx, value)| **value)
            .expect("maxing a non-empty list always produces something");

        Ok((left, right))
    }
}

pub fn part1(input: &Path) -> Result<()> {
    let total_output_joltage = parse::<Bank>(input)?
        .map(|bank| -> Result<_> {
            let (left, right) = bank.select_indices_pt1()?;
            let joltage = bank.joltage_from_indices(left, right);
            Ok(joltage)
        })
        .try_fold(0, |acc, elem| -> Result<_> { Ok(elem? + acc) })?;
    println!("total output joltage: {total_output_joltage}");
    Ok(())
}

pub fn part2(input: &Path) -> Result<()> {
    unimplemented!("input file: {:?}", input)
}
