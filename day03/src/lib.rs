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
    fn joltage_from_indices<const N: usize>(&self, indices: [usize; N]) -> u64 {
        debug_assert!(
            indices.windows(2).all(|window| window[0] < window[1]),
            "each subsequent index must increase"
        );
        indices
            .iter()
            .copied()
            .rev()
            .enumerate()
            .map(|(exponent, index)| {
                let exponent = exponent as u32;
                let value = self.0[index] as u64;
                10_u64.pow(exponent) * value
            })
            .sum()
    }

    fn select_indices<const N: usize>(&self) -> Result<[usize; N]> {
        if self.0.len() < N {
            return Err(eyre!("bank has too few batteries"));
        }

        // initialization loop
        let mut indices = [0; N];
        for index in 0..N {
            let mut initial_iter = self.0.iter().enumerate().rev().skip(N - 1 - index);
            let mut adapted_iter;
            let iter = if index > 0 {
                let previous_index = indices[index - 1];
                adapted_iter = initial_iter.take_while(move |(idx, _value)| *idx > previous_index);
                &mut adapted_iter as &mut dyn Iterator<Item = _>
            } else {
                &mut initial_iter
            };

            indices[index] = iter
                .max_by_key(|(_idx, value)| **value)
                .expect("maxing a non-empty list always produces something")
                .0;
        }

        Ok(indices)
    }
}

fn solve<const N: usize>(input: &Path, part_n: u8) -> Result<()> {
    let total_output_joltage = parse::<Bank>(input)?
        .map(|bank| -> Result<_> {
            bank.select_indices::<N>()
                .map(|indices| bank.joltage_from_indices(indices))
        })
        .try_fold(0, |acc, elem| -> Result<_> { Ok(elem? + acc) })?;
    println!("total output joltage (part {part_n}): {total_output_joltage}");
    Ok(())
}

pub fn part1(input: &Path) -> Result<()> {
    solve::<2>(input, 1)
}

pub fn part2(input: &Path) -> Result<()> {
    solve::<12>(input, 2)
}
