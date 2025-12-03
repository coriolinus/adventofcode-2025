use aoclib::parse;
use color_eyre::{
    eyre::{eyre, OptionExt, Report},
    Result,
};
use std::{mem::MaybeUninit, path::Path, str::FromStr};

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

        // each input has 100 numbers. Just iterating over each 12-combination is untenable; there are ~1e15 such.
        // let's just try naively using exactly the same strategy as in part 1, writ large

        // this kind of allocation-elimination microoptimization is honestly kind of pointless; this runs fast enough.
        // but on the other hand, it's a chance to learn something new, so...

        // initialization loop
        let mut indices: [MaybeUninit<usize>; N] = [MaybeUninit::uninit(); N];
        for index in 0..N {
            let mut initial_iter = self.0.iter().enumerate().rev().skip(N - 1 - index);
            let mut adapted_iter;
            let iter = if index > 0 {
                // SAFETY: we choose the indices in order and only refer to a previous one
                let previous_index = unsafe { indices[index - 1].assume_init() };
                adapted_iter = initial_iter.take_while(move |(idx, _value)| *idx > previous_index);
                &mut adapted_iter as &mut dyn Iterator<Item = _>
            } else {
                &mut initial_iter
            };

            indices[index].write(
                iter.max_by_key(|(_idx, value)| **value)
                    .expect("maxing a non-empty list always produces something")
                    .0,
            );
        }

        // safety: initializing an array by element is literally in the MaybeUninit docs:
        // https://doc.rust-lang.org/stable/std/mem/union.MaybeUninit.html
        // and I'm basically stealing the implementation from the (unstable) `MaybeUninit::array_assume_init`:
        // https://github.com/joboet/rust/blob/985071b08f5c03e4f18d43c15f3ea82395588a5e/library/core/src/mem/maybe_uninit.rs#L843
        Ok(unsafe { (&indices as *const _ as *const [usize; N]).read() })
    }
}

pub fn part1(input: &Path) -> Result<()> {
    let total_output_joltage = parse::<Bank>(input)?
        .map(|bank| -> Result<_> {
            let indices = bank.select_indices::<2>()?;
            let joltage = bank.joltage_from_indices(indices);
            Ok(joltage)
        })
        .try_fold(0, |acc, elem| -> Result<_> { Ok(elem? + acc) })?;
    println!("total output joltage: {total_output_joltage}");
    Ok(())
}

pub fn part2(input: &Path) -> Result<()> {
    let total_output_joltage = parse::<Bank>(input)?
        .map(|bank| -> Result<_> {
            let indices = bank.select_indices::<12>()?;
            let joltage = bank.joltage_from_indices(indices);
            Ok(joltage)
        })
        .try_fold(0, |acc, elem| -> Result<_> { Ok(elem? + acc) })?;
    println!("total output joltage (pt2): {total_output_joltage}");
    Ok(())
}
