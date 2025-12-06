use color_eyre::{
    eyre::{eyre, Context, OptionExt},
    Result,
};
use itertools::{Itertools, Position};
use std::{
    io::{BufRead, BufReader},
    path::Path,
};

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
                    let row = row
                        .split_whitespace()
                        .map(|value| value.parse::<u64>().map_err(Into::into))
                        .collect::<Result<Vec<_>>>()?;
                    rows.push(row);
                }
                Position::Last => {
                    operations = row
                        .split_whitespace()
                        .map(|value| value.parse::<Operation>().map_err(Into::into))
                        .collect::<Result<_>>()?;
                }
            }
        }

        if rows.iter().any(|row| row.len() != operations.len()) {
            return Err(eyre!("not all rows and operations had the same width"));
        }

        Ok(Self { rows, operations })
    }

    fn problems(&self) -> impl '_ + Iterator<Item = Problem> {
        self.operations
            .iter()
            .copied()
            .enumerate()
            .map(|(column, operation)| {
                let values = (0..self.rows.len())
                    .map(|row| self.rows[row][column])
                    .collect();
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

struct InputPt2 {
    problems: Vec<Problem>,
}

impl InputPt2 {
    fn parse_problem(
        lines: &[Vec<u8>],
        operation: Operation,
        range: impl Iterator<Item = usize>,
    ) -> Result<Problem> {
        let mut problem = Problem {
            operation,
            values: Vec::new(),
        };

        let mut consecutive_empty_lines = 0;
        for byte_column in range {
            let value = lines
                .iter()
                .map(|line| {
                    line.get(byte_column)
                        .map(|byte| *byte as char)
                        .unwrap_or(' ')
                })
                .collect::<String>();
            let value = value.trim();
            if value.is_empty() {
                consecutive_empty_lines += 1;
                if consecutive_empty_lines > 1 {
                    break;
                }
                continue;
            } else {
                consecutive_empty_lines = 0;
            }
            problem
                .values
                .push(value.parse().wrap_err("invalid value")?);
        }

        Ok(problem)
    }

    fn parse(input: &Path) -> Result<Self> {
        let file = std::fs::File::open(input)?;
        let reader = BufReader::new(file);
        let mut lines = reader
            .lines()
            .map(|line| line.map(String::into_bytes).map_err(Into::into))
            .collect::<Result<Vec<_>>>()?;
        let operations_line = lines.pop().ok_or_eyre("no operations line in input")?;

        let mut problems = Vec::new();
        let mut end_of_previous = 0;
        for (idx, byte) in operations_line.iter().copied().enumerate().skip(1) {
            if byte.is_ascii_whitespace() {
                continue; // not a new field
            }
            let operation = match operations_line[end_of_previous] {
                b'+' => Operation::Sum,
                b'*' => Operation::Product,
                c => return Err(eyre!("invalid operation: {}", c as char)),
            };

            problems.push(Self::parse_problem(
                &lines,
                operation,
                end_of_previous..idx,
            )?);
            end_of_previous = idx;
        }

        // of course we have not yet pushed the trailing values
        let operation = match operations_line[end_of_previous] {
            b'+' => Operation::Sum,
            b'*' => Operation::Product,
            c => return Err(eyre!("invalid operation: {}", c as char)),
        };
        problems.push(Self::parse_problem(&lines, operation, end_of_previous..)?);

        Ok(Self { problems })
    }
}

pub fn part2(input: &Path) -> Result<()> {
    let input = InputPt2::parse(input)?;
    let grand_total = input
        .problems
        .iter()
        .map(|problem| problem.solve())
        .sum::<u64>();
    println!("grand total (pt2): {grand_total}");
    Ok(())
}
