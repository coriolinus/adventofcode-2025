use color_eyre::{Result, eyre::{Context, eyre}};
use itertools::{Itertools, Position};
use std::{io::{BufRead, BufReader}, path::Path};

#[derive(Debug, Clone, Copy, PartialEq, Eq, parse_display::FromStr, parse_display::Display)]
enum Operation {
    #[display("+")]
    Sum,
    #[display("*")]
    Product,
}

struct Input {
    rows: Vec<Vec<u64>>,
    operations: Vec<Operation>,
}

impl Input {
    fn parse(input: &Path) -> Result<Self> {
        let file = std::fs::File::open(input)?;
        let reader = BufReader::new(file);

        let mut rows = Vec::new();
        let mut operations = Vec::new();

        for (position, row) in reader.lines().with_position() {
            let row = row.wrap_err_with(|| format!("{position:?} row could not be read"))?;
            match position {
                Position::Only => return Err(eyre!("not enough rows in input")),
                Position::First | Position::Middle => {
                    let row = row.split_whitespace().map(|value| value.parse::<u64>().map_err(Into::into)).collect::<Result<Vec<_>>>()?;
                    rows.push(row);
                }
                Position::Last => {
                    operations = row.split_whitespace().map(|value| value.parse::<Operation>().map_err(Into::into)).collect::<Result<_>>()?;
                }
            }
        }

        if rows.iter().any(|row| row.len() != operations.len()) {
            return Err(eyre!("not all rows and operations had the same width"));
        }

        Ok(Self { rows, operations })
    }

    fn problems(&self) -> impl '_ + Iterator<Item = Problem> {
        self.operations.iter().copied().enumerate().map(|(column, operation)| {
            let values = (0..self.rows.len()).map(|row| self.rows[row][column]).collect();
            Problem { operation, values } 
        })
    }
}

struct Problem {
    operation: Operation,
    values: Vec<u64>,
}

impl Problem {
    fn solve(&self) -> u64 {
        match self.operation {
            Operation::Sum => self.values.iter().sum(),
            Operation::Product => self.values.iter().product(),
        }
    }
}

pub fn part1(input: &Path) -> Result<()> {
    let input = Input::parse(input)?;
    let grand_total = input.problems().map(|problem| problem.solve()).sum::<u64>();
    println!("grand total: {grand_total}");
    Ok(())
}

pub fn part2(input: &Path) -> Result<()> {
    unimplemented!("input file: {:?}", input)
}
