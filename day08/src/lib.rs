use aoclib::parse;
use color_eyre::Result;
use std::{collections::BTreeMap, path::Path};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    parse_display::Display,
    parse_display::FromStr,
    derive_more::Add,
    derive_more::Sub,
)]
#[display("{x},{y},{z}")]
struct Point {
    x: u32,
    y: u32,
    z: u32,
}

impl Point {
    fn distance_square(&self, other: &Self) -> u64 {
        let dx = self.x.abs_diff(other.x) as u64;
        let dy = self.y.abs_diff(other.y) as u64;
        let dz = self.z.abs_diff(other.z) as u64;

        dx * dx + dy * dy + dz * dz
    }
}

/// Solve part 1
///
/// - compute distances between each point pair
/// - considering point pairs in order by distance, connect them
fn solve_part1(points: &[Point], connection_limit: usize) -> u64 {
    // keep track of the pairs, by distance
    let mut pairs_by_distance = BTreeMap::<_, Vec<_>>::new();
    for (i, a) in points.iter().enumerate() {
        for (j, b) in points.iter().take(i).enumerate() {
            let distance = a.distance_square(b);
            pairs_by_distance.entry(distance).or_default().push((i, j));
        }
    }

    // each point starts assigned to its own circuit, where the circuit id is the index of that point in points
    let mut circuit_assignments = (0..points.len()).collect::<Vec<_>>();

    for (i, j) in pairs_by_distance
        .values()
        .flat_map(|list| list.iter())
        .take(connection_limit)
        .copied()
    {
        // eprintln!("considering ({}, {})", points[j], points[i]);
        if circuit_assignments[i] == circuit_assignments[j] {
            // not actually a new connection
            // skip further work
            // eprintln!(
            //     " already part of the same circuit: {}",
            //     circuit_assignments[i]
            // );
            continue;
        }
        // this _is_ a new connection, which we implement by reassigning the circuit
        // assignment for everything on the bigger circuit to the smaller circuit
        // this is a linear time operation on the number of points
        // we could improve on that by keeping a map of indices by circuit id,
        // but for now I'd save the hassle
        debug_assert_ne!(
            i, j,
            "we should never hit the same circuit twice in this loop"
        );
        let bigger = circuit_assignments[i].max(circuit_assignments[j]);
        let smaller = circuit_assignments[i].min(circuit_assignments[j]);
        // eprintln!(
        //     " reassigning all `{bigger}` ({}) circuits to `{smaller}` ({})",
        //     points[bigger], points[smaller]
        // );
        for circuit_assignment in circuit_assignments.iter_mut() {
            if *circuit_assignment == bigger {
                *circuit_assignment = smaller;
            }
        }
        // eprintln!(" circuit assignments: {circuit_assignments:?}");
    }

    // we need the product of the sizes of the greatest three circuits
    let mut circuit_sizes = vec![0_u64; points.len()];
    for circuit in circuit_assignments.iter().copied() {
        circuit_sizes[circuit] += 1;
    }
    // eprintln!("circuit sizes: {circuit_sizes:?}");
    circuit_sizes.sort();
    circuit_sizes.into_iter().rev().take(3).product()
}

pub fn part1(input: &Path, connection_limit: usize) -> Result<()> {
    let points = parse::<Point>(input)?.collect::<Vec<_>>();
    let circuit_size_product = solve_part1(&points, connection_limit);
    println!("product of size of 3 largest circuits: {circuit_size_product}");
    Ok(())
}

pub fn part2(input: &Path) -> Result<()> {
    unimplemented!("input file: {:?}", input)
}
